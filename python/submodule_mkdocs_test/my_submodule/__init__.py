from submodule_mkdocs_test._submodule_mkdocs_test._my_submodule import Thing
import submodule_mkdocs_test._submodule_mkdocs_test._my_submodule

__doc__ = submodule_mkdocs_test._submodule_mkdocs_test._my_submodule.__doc__
if hasattr(submodule_mkdocs_test._submodule_mkdocs_test._my_submodule, "__all__"):
    __all__ = submodule_mkdocs_test._submodule_mkdocs_test._my_submodule.__all__

print("hello from submodule_mkdocs_test/my_submodule/__init__.py")