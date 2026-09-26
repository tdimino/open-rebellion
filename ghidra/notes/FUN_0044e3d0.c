
void __thiscall FUN_0044e3d0(void *this,int param_1)

{
  if (*(int *)((int)this + 0x17c) != param_1) {
    switch(param_1) {
    case 0x65:
      (**(code **)(**(int **)((int)this + 0x120) + 8))();
      SetFocus(*(HWND *)((int)this + 0x18));
      (**(code **)(**(int **)((int)this + 0x160) + 8))();
      (**(code **)(**(int **)((int)this + 0x168) + 8))();
      (**(code **)(**(int **)((int)this + 0x16c) + 8))();
      FUN_006075e0(this,1);
      *(int *)((int)this + 0x17c) = param_1;
      return;
    case 0x66:
    case 0x67:
      FUN_0044e500(this,*(int *)((int)this + 0x180),1);
      (**(code **)(**(int **)((int)this + 0x120) + 4))(5);
      SetFocus(*(HWND *)(*(int *)((int)this + 0x120) + 0x18));
      (**(code **)(**(int **)((int)this + 0x160) + 4))(5);
      (**(code **)(**(int **)((int)this + 0x168) + 8))();
      (**(code **)(**(int **)((int)this + 0x16c) + 8))();
      *(int *)((int)this + 0x17c) = param_1;
      return;
    case 0x68:
      (**(code **)(**(int **)((int)this + 0x120) + 8))();
      SetFocus(*(HWND *)((int)this + 0x18));
      (**(code **)(**(int **)((int)this + 0x160) + 8))();
      (**(code **)(**(int **)((int)this + 0x168) + 4))(5);
      (**(code **)(**(int **)((int)this + 0x16c) + 4))(5);
      FUN_006075e0(this,5);
    }
    *(int *)((int)this + 0x17c) = param_1;
  }
  return;
}

