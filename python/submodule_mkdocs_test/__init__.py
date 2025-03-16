from ._submodule_mkdocs_test import *
import submodule_mkdocs_test._submodule_mkdocs_test

__doc__ = submodule_mkdocs_test._submodule_mkdocs_test.__doc__
if hasattr(submodule_mkdocs_test._submodule_mkdocs_test, "__all__"):
    __all__ = submodule_mkdocs_test._submodule_mkdocs_test.__all__

print("hello from submodule_mkdocs_test/__init__.py")