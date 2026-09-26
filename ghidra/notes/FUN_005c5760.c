
void __thiscall FUN_005c5760(void *this,uint param_1)

{
  int *piVar1;
  int *piVar2;
  
  if (param_1 != *(uint *)((int)this + 0x9c)) {
    piVar1 = (int *)FUN_00604500((void *)((int)this + 0x6c),*(uint *)((int)this + 0x9c));
    if (param_1 == 0) {
      param_1 = *(uint *)((int)this + 0xa0);
    }
    piVar2 = (int *)FUN_00604500((void *)((int)this + 0x6c),param_1);
    if (piVar1 != (int *)0x0) {
      SetWindowPos((HWND)piVar1[6],(HWND)0x0,600,600,0,0,1);
      (**(code **)(*piVar1 + 8))();
      *(undefined4 *)((int)this + 0xa0) = *(undefined4 *)((int)this + 0x9c);
    }
    if (piVar2 != (int *)0x0) {
      SetWindowPos((HWND)piVar2[6],(HWND)0x0,0,0,0,0,1);
      (**(code **)(*piVar2 + 4))(5);
      *(int *)((int)this + 0x9c) = piVar2[9];
    }
  }
  return;
}

