import fast_paths

print("=== Basic Usage ===")
input_graph = fast_paths.PyInputGraph()
input_graph.add_edge(0, 6, 12)
input_graph.add_edge(6, 8, 5)
input_graph.add_edge(0, 8, 100)
input_graph.add_edge(1, 8, 20)
input_graph.freeze()

prepared = fast_paths.prepare(input_graph)

path = prepared.calc_path(0, 8)
if path and path.is_found():
    print(f"Shortest path from 0 to 8: weight={path.get_weight()}, nodes={path.get_nodes()}")
    print(path)

print("\n=== Reusable Calculator ===")
calc = fast_paths.create_calculator(prepared)
print("0→8:", calc.calc_path(prepared, 0, 8))
print("1→8:", calc.calc_path(prepared, 1, 8))

print("\n=== Multiple Sources & Targets ===")
sources = [(0, 0), (1, 10)]
targets = [(8, 0), (6, 5)]
multi = calc.calc_path_multiple_sources_and_targets(prepared, sources, targets)
if multi and multi.is_found():
    print(f"Best multi-path: weight={multi.get_weight()}, nodes={multi.get_nodes()}")

print("\n=== Custom Params ===")
params = fast_paths.PyParams(hierarchy_depth_factor=0.6)
prepared2 = fast_paths.prepare_with_params(input_graph, params)
print(f"Prepared with params (nodes: {prepared2.get_num_nodes()})")

print("\n=== Prepare with Ordering ===")
ordering = prepared.get_node_ordering()
prepared3 = fast_paths.prepare_with_order(input_graph, ordering)
print(f"Re-prepared with ordering (nodes: {prepared3.get_num_nodes()})")

print("\nAll examples completed successfully!")
