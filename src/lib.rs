use ::fast_paths::{
    Edge, FastGraph, FastGraph32, FastGraphBuilder, InputGraph, Params, ParamsWithOrder,
    PathCalculator, ShortestPath, WEIGHT_MAX, calc_path, calc_path_multiple_sources_and_targets,
    create_calculator, prepare, prepare_with_order, prepare_with_order_with_params,
    prepare_with_params,
};
use pyo3::prelude::*;
use std::sync::Arc;

type NodeId = usize;
type Weight = usize;

// MARK: Edge
#[pyclass(skip_from_py_object)]
#[derive(Clone)]
struct PyEdge {
    inner: Edge,
}

#[pymethods]
impl PyEdge {
    fn from_node(&self) -> NodeId {
        self.inner.from
    }

    fn to_node(&self) -> NodeId {
        self.inner.to
    }

    fn weight(&self) -> Weight {
        self.inner.weight
    }

    fn __repr__(&self) -> String {
        format!(
            "Edge(from={}, to={}, weight={})",
            self.from_node(),
            self.to_node(),
            self.weight()
        )
    }
}

impl From<Edge> for PyEdge {
    fn from(inner: Edge) -> Self {
        Self { inner }
    }
}

// MARK: InputGraph
#[pyclass(skip_from_py_object)]
#[derive(Clone)]
struct PyInputGraph {
    inner: InputGraph,
}

#[pymethods]
impl PyInputGraph {
    #[new]
    fn new() -> Self {
        Self {
            inner: InputGraph::new(),
        }
    }

    fn add_edge(&mut self, from: NodeId, to: NodeId, weight: Weight) -> usize {
        self.inner.add_edge(from, to, weight)
    }

    fn add_edge_bidir(&mut self, from: NodeId, to: NodeId, weight: Weight) -> usize {
        self.inner.add_edge_bidir(from, to, weight)
    }

    fn freeze(&mut self) {
        self.inner.freeze();
    }

    fn thaw(&mut self) {
        self.inner.thaw();
    }

    fn get_num_nodes(&self) -> usize {
        self.inner.get_num_nodes()
    }

    fn get_num_edges(&self) -> usize {
        self.inner.get_num_edges()
    }

    fn get_edges(&self) -> Vec<PyEdge> {
        self.inner
            .get_edges()
            .iter()
            .cloned()
            .map(PyEdge::from)
            .collect()
    }

    fn to_file(&self, filename: &str) -> PyResult<()> {
        self.inner
            .to_file(filename)
            .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))
    }

    #[staticmethod]
    fn from_file(filename: &str) -> Self {
        Self {
            inner: InputGraph::from_file(filename),
        }
    }

    fn to_dimacs_file(&self, filename: &str) -> PyResult<()> {
        self.inner
            .to_dimacs_file(filename)
            .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))
    }

    #[staticmethod]
    fn from_dimacs_file(filename: &str) -> Self {
        Self {
            inner: InputGraph::from_dimacs_file(filename),
        }
    }

    fn unit_test_output_string(&self) -> String {
        self.inner.unit_test_output_string()
    }
}

// MARK: FastGraph
#[pyclass]
struct PyFastGraph {
    inner: Arc<FastGraph>,
}

#[pymethods]
impl PyFastGraph {
    fn calc_path(&self, source: NodeId, target: NodeId) -> Option<PyShortestPath> {
        calc_path(&self.inner, source, target).map(PyShortestPath::from)
    }

    fn get_node_ordering(&self) -> Vec<NodeId> {
        self.inner.get_node_ordering()
    }

    fn get_num_nodes(&self) -> usize {
        self.inner.get_num_nodes()
    }

    fn get_num_out_edges(&self) -> usize {
        self.inner.get_num_out_edges()
    }

    fn get_num_in_edges(&self) -> usize {
        self.inner.get_num_in_edges()
    }

    fn begin_in_edges(&self, node: NodeId) -> usize {
        self.inner.begin_in_edges(node)
    }

    fn end_in_edges(&self, node: NodeId) -> usize {
        self.inner.end_in_edges(node)
    }

    fn begin_out_edges(&self, node: NodeId) -> usize {
        self.inner.begin_out_edges(node)
    }

    fn end_out_edges(&self, node: NodeId) -> usize {
        self.inner.end_out_edges(node)
    }
}

// MARK: FastGraph32
#[allow(dead_code)]
#[pyclass]
struct PyFastGraph32 {
    inner: Arc<FastGraph32>,
}

// MARK: ShortestPath
#[pyclass(skip_from_py_object)]
#[derive(Clone)]
struct PyShortestPath {
    inner: ShortestPath,
}

#[pymethods]
impl PyShortestPath {
    fn get_weight(&self) -> Weight {
        self.inner.get_weight()
    }

    fn get_nodes(&self) -> Vec<NodeId> {
        self.inner.get_nodes().clone()
    }

    fn get_source(&self) -> NodeId {
        self.inner.get_source()
    }

    fn get_target(&self) -> NodeId {
        self.inner.get_target()
    }

    fn is_found(&self) -> bool {
        self.inner.is_found()
    }

    fn __repr__(&self) -> String {
        if self.is_found() {
            format!(
                "ShortestPath(source={}, target={}, weight={}, nodes={:?})",
                self.get_source(),
                self.get_target(),
                self.get_weight(),
                self.get_nodes()
            )
        } else {
            format!(
                "ShortestPath(source={}, target={}, not found)",
                self.get_source(),
                self.get_target()
            )
        }
    }
}

impl From<ShortestPath> for PyShortestPath {
    fn from(inner: ShortestPath) -> Self {
        Self { inner }
    }
}

// MARK: PathCalculator
#[pyclass]
struct PyPathCalculator {
    inner: PathCalculator,
}

#[pymethods]
impl PyPathCalculator {
    #[new]
    fn new(num_nodes: usize) -> Self {
        Self {
            inner: PathCalculator::new(num_nodes),
        }
    }

    fn calc_path(
        &mut self,
        py_graph: &PyFastGraph,
        source: NodeId,
        target: NodeId,
    ) -> Option<PyShortestPath> {
        self.inner
            .calc_path(&py_graph.inner, source, target)
            .map(PyShortestPath::from)
    }

    fn calc_path_multiple_sources_and_targets(
        &mut self,
        py_graph: &PyFastGraph,
        sources: Vec<(NodeId, Weight)>,
        targets: Vec<(NodeId, Weight)>,
    ) -> Option<PyShortestPath> {
        self.inner
            .calc_path_multiple_sources_and_targets(&py_graph.inner, sources, targets)
            .map(PyShortestPath::from)
    }
}

// MARK: Params
#[pyclass]
struct PyParams {
    inner: Params,
}

#[pymethods]
impl PyParams {
    #[new]
    #[pyo3(signature = (hierarchy_depth_factor = 0.5, edge_quotient_factor = 1.0,
                       max_initial = 100, max_neighbor = 200, max_contraction = 300))]
    fn new(
        hierarchy_depth_factor: f32,
        edge_quotient_factor: f32,
        max_initial: usize,
        max_neighbor: usize,
        max_contraction: usize,
    ) -> Self {
        let mut p = Params::default();
        p.hierarchy_depth_factor = hierarchy_depth_factor;
        p.edge_quotient_factor = edge_quotient_factor;
        p.max_settled_nodes_initial_relevance = max_initial;
        p.max_settled_nodes_neighbor_relevance = max_neighbor;
        p.max_settled_nodes_contraction = max_contraction;
        Self { inner: p }
    }

    #[staticmethod]
    fn default() -> Self {
        Self {
            inner: Params::default(),
        }
    }

    fn get_hierarchy_depth_factor(&self) -> f32 {
        self.inner.hierarchy_depth_factor
    }
    fn set_hierarchy_depth_factor(&mut self, v: f32) {
        self.inner.hierarchy_depth_factor = v;
    }

    fn get_edge_quotient_factor(&self) -> f32 {
        self.inner.edge_quotient_factor
    }
    fn set_edge_quotient_factor(&mut self, v: f32) {
        self.inner.edge_quotient_factor = v;
    }

    fn get_max_settled_nodes_initial_relevance(&self) -> usize {
        self.inner.max_settled_nodes_initial_relevance
    }
    fn set_max_settled_nodes_initial_relevance(&mut self, v: usize) {
        self.inner.max_settled_nodes_initial_relevance = v;
    }

    fn get_max_settled_nodes_neighbor_relevance(&self) -> usize {
        self.inner.max_settled_nodes_neighbor_relevance
    }
    fn set_max_settled_nodes_neighbor_relevance(&mut self, v: usize) {
        self.inner.max_settled_nodes_neighbor_relevance = v;
    }

    fn get_max_settled_nodes_contraction(&self) -> usize {
        self.inner.max_settled_nodes_contraction
    }
    fn set_max_settled_nodes_contraction(&mut self, v: usize) {
        self.inner.max_settled_nodes_contraction = v;
    }
}

// MARK: ParamsWithOrder
#[pyclass]
struct PyParamsWithOrder {
    inner: ParamsWithOrder,
}

#[pymethods]
impl PyParamsWithOrder {
    #[new]
    fn new(max_settled_nodes_contraction_with_order: usize) -> Self {
        let mut p = ParamsWithOrder::default();
        p.max_settled_nodes_contraction_with_order = max_settled_nodes_contraction_with_order;
        Self { inner: p }
    }

    fn get_max_settled_nodes_contraction_with_order(&self) -> usize {
        self.inner.max_settled_nodes_contraction_with_order
    }

    fn set_max_settled_nodes_contraction_with_order(&mut self, v: usize) {
        self.inner.max_settled_nodes_contraction_with_order = v;
    }
}

// MARK: FastGraphBuilder
#[allow(dead_code)]
#[pyclass]
struct PyFastGraphBuilder {
    inner: FastGraphBuilder,
}

// MARK: Free functions
#[pyfunction(name = "prepare")]
fn py_prepare(input_graph: &PyInputGraph) -> PyFastGraph {
    PyFastGraph {
        inner: Arc::new(prepare(&input_graph.inner)),
    }
}

#[pyfunction(name = "prepare_with_params")]
fn py_prepare_with_params(input_graph: &PyInputGraph, params: &PyParams) -> PyFastGraph {
    PyFastGraph {
        inner: Arc::new(prepare_with_params(&input_graph.inner, &params.inner)),
    }
}

#[pyfunction(name = "prepare_with_order")]
fn py_prepare_with_order(input_graph: &PyInputGraph, order: Vec<NodeId>) -> PyResult<PyFastGraph> {
    prepare_with_order(&input_graph.inner, &order)
        .map(|g| PyFastGraph { inner: Arc::new(g) })
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e))
}

#[pyfunction(name = "prepare_with_order_with_params")]
fn py_prepare_with_order_with_params(
    input_graph: &PyInputGraph,
    order: Vec<NodeId>,
    params: &PyParamsWithOrder,
) -> PyResult<PyFastGraph> {
    prepare_with_order_with_params(&input_graph.inner, &order, &params.inner)
        .map(|g| PyFastGraph { inner: Arc::new(g) })
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e))
}

#[pyfunction(name = "create_calculator")]
fn py_create_calculator(py_graph: &PyFastGraph) -> PyPathCalculator {
    PyPathCalculator {
        inner: create_calculator(&py_graph.inner),
    }
}

#[pyfunction(name = "calc_path")]
fn py_calc_path(py_graph: &PyFastGraph, source: NodeId, target: NodeId) -> Option<PyShortestPath> {
    calc_path(&py_graph.inner, source, target).map(PyShortestPath::from)
}

#[pyfunction(name = "calc_path_multiple_sources_and_targets")]
fn py_calc_path_multiple_sources_and_targets(
    py_graph: &PyFastGraph,
    sources: Vec<(NodeId, Weight)>,
    targets: Vec<(NodeId, Weight)>,
) -> Option<PyShortestPath> {
    calc_path_multiple_sources_and_targets(&py_graph.inner, sources, targets)
        .map(PyShortestPath::from)
}

// MARK: Module
#[pymodule]
fn fast_paths(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyEdge>()?;
    m.add_class::<PyInputGraph>()?;
    m.add_class::<PyFastGraph>()?;
    m.add_class::<PyFastGraph32>()?;
    m.add_class::<PyFastGraphBuilder>()?;
    m.add_class::<PyShortestPath>()?;
    m.add_class::<PyPathCalculator>()?;
    m.add_class::<PyParams>()?;
    m.add_class::<PyParamsWithOrder>()?;

    m.add_function(wrap_pyfunction!(py_prepare, m)?)?;
    m.add_function(wrap_pyfunction!(py_prepare_with_params, m)?)?;
    m.add_function(wrap_pyfunction!(py_prepare_with_order, m)?)?;
    m.add_function(wrap_pyfunction!(py_prepare_with_order_with_params, m)?)?;
    m.add_function(wrap_pyfunction!(py_create_calculator, m)?)?;
    m.add_function(wrap_pyfunction!(py_calc_path, m)?)?;
    m.add_function(wrap_pyfunction!(
        py_calc_path_multiple_sources_and_targets,
        m
    )?)?;

    m.add("WEIGHT_MAX", WEIGHT_MAX)?;

    Ok(())
}
