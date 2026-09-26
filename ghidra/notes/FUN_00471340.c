
void __thiscall FUN_00471340(void *this,int param_1)

{
  if (*(int *)((int)this + 0x17c) != param_1) {
    switch(param_1) {
    case 0x6f:
      SetFocus(*(HWND *)((int)this + 0x18));
      if (*(int **)((int)this + 0x168) != (int *)0x0) {
        (**(code **)(**(int **)((int)this + 0x168) + 8))();
      }
      (**(code **)(**(int **)((int)this + 0x16c) + 8))();
      (**(code **)(**(int **)((int)this + 0x120) + 8))();
      (**(code **)(**(int **)((int)this + 0x160) + 8))();
      FUN_006075e0(this,1);
      *(int *)((int)this + 0x17c) = param_1;
      return;
    case 0x70:
      if (*(int **)((int)this + 0x168) != (int *)0x0) {
        (**(code **)(**(int **)((int)this + 0x168) + 8))();
      }
      (**(code **)(**(int **)((int)this + 0x16c) + 8))();
      FUN_004714e0(this,*(int *)((int)this + 0x180),1);
      (**(code **)(**(int **)((int)this + 0x120) + 4))(5);
      SetFocus(*(HWND *)(*(int *)((int)this + 0x120) + 0x18));
      (**(code **)(**(int **)((int)this + 0x160) + 4))(5);
      *(int *)((int)this + 0x17c) = param_1;
      return;
    case 0x71:
      if (*(int **)((int)this + 0x168) != (int *)0x0) {
        (**(code **)(**(int **)((int)this + 0x168) + 8))();
      }
      (**(code **)(**(int **)((int)this + 0x16c) + 8))();
      FUN_004714e0(this,*(int *)((int)this + 0x180),1);
      (**(code **)(**(int **)((int)this + 0x120) + 4))(5);
      SetFocus(*(HWND *)(*(int *)((int)this + 0x120) + 0x18));
      (**(code **)(**(int **)((int)this + 0x160) + 4))(5);
      *(int *)((int)this + 0x17c) = param_1;
      return;
    case 0x72:
      SetFocus(*(HWND *)((int)this + 0x18));
      (**(code **)(**(int **)((int)this + 0x120) + 8))();
      (**(code **)(**(int **)((int)this + 0x160) + 8))();
      FUN_006075e0(this,4);
      if (*(int **)((int)this + 0x168) != (int *)0x0) {
        (**(code **)(**(int **)((int)this + 0x168) + 4))(5);
      }
      (**(code **)(**(int **)((int)this + 0x16c) + 4))(5);
    }
    *(int *)((int)this + 0x17c) = param_1;
  }
  return;
}

