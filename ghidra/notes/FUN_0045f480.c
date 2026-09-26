
void __thiscall FUN_0045f480(void *this,int param_1)

{
  int iVar1;
  int *piVar2;
  char *pcVar3;
  
  iVar1 = param_1;
  if (param_1 == 1) {
    FUN_006075e0(this,2);
    (**(code **)(**(int **)((int)this + 0x13c) + 8))();
    (**(code **)(**(int **)((int)this + 0x144) + 8))();
    (**(code **)(**(int **)((int)this + 0x140) + 8))();
    (**(code **)(**(int **)((int)this + 300) + 4))(5);
    (**(code **)(**(int **)((int)this + 0x128) + 4))(5);
    (**(code **)(**(int **)((int)this + 0x124) + 4))(5);
    piVar2 = (int *)FUN_00604500((void *)(*(int *)((int)this + 0x11c) + 0x6c),0x67);
    (**(code **)(*piVar2 + 0xc))();
    piVar2 = (int *)FUN_00604500((void *)(*(int *)((int)this + 0x11c) + 0x6c),0x68);
    (**(code **)(*piVar2 + 0x10))();
    if (*(int *)((int)this + 0x148) != 0) {
      FUN_00609500(*(void **)((int)this + 300),*(int *)(*(int *)((int)this + 0x148) + 0xc),0);
      FUN_00609ed0(*(void **)((int)this + 300),*(int *)(*(int *)((int)this + 0x148) + 0xc));
      FUN_00605070(*(void **)((int)this + 0x128));
      pcVar3 = (char *)FUN_00583c40(*(int *)((int)this + 0x148) + 0x14);
      FUN_00604f90(*(void **)((int)this + 0x128),pcVar3);
      FUN_00605110(*(void **)((int)this + 0x128),0,0);
    }
    param_1 = CONCAT22(DAT_0065d424,0x1842);
    FUN_006019a0(*(void **)((int)this + 0x120),&param_1);
  }
  else if (param_1 == 2) {
    if (*(int *)((int)this + 0x148) == 0) {
      return;
    }
    FUN_006075e0(this,1);
    (**(code **)(**(int **)((int)this + 300) + 8))();
    (**(code **)(**(int **)((int)this + 0x128) + 8))();
    (**(code **)(**(int **)((int)this + 0x124) + 8))();
    (**(code **)(**(int **)((int)this + 0x13c) + 4))(5);
    SetFocus(*(HWND *)(*(int *)((int)this + 0x13c) + 0x18));
    (**(code **)(**(int **)((int)this + 0x140) + 4))(5);
    (**(code **)(**(int **)((int)this + 0x144) + 4))(5);
    piVar2 = (int *)FUN_00604500((void *)(*(int *)((int)this + 0x11c) + 0x6c),0x67);
    (**(code **)(*piVar2 + 0x10))();
    piVar2 = (int *)FUN_00604500((void *)(*(int *)((int)this + 0x11c) + 0x6c),0x68);
    (**(code **)(*piVar2 + 0xc))();
    FUN_0045fa60(this);
  }
  *(int *)((int)this + 0x114) = iVar1;
  InvalidateRect(*(HWND *)((int)this + 0x18),(RECT *)0x0,0);
  return;
}

