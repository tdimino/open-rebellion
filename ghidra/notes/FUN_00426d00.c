
void __thiscall FUN_00426d00(void *this,int param_1)

{
  undefined4 uVar1;
  int *piVar2;
  void *pvVar3;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_0062c0cb;
  pvStack_c = ExceptionList;
  uVar1 = *(undefined4 *)((int)this + 0x1c);
  ExceptionList = &pvStack_c;
  FUN_006037f0(7);
  if (*(int *)((int)this + 0x124) != 0) {
    FUN_00600280(*(int *)((int)this + 0x124));
    if (*(undefined4 **)((int)this + 0x124) != (undefined4 *)0x0) {
      (**(code **)**(undefined4 **)((int)this + 0x124))(1);
    }
    *(undefined4 *)((int)this + 0x124) = 0;
  }
  piVar2 = *(int **)((int)this + 0x248);
  if (param_1 == 0x80) {
    if (piVar2 != (int *)0x0) {
      (**(code **)(*piVar2 + 8))();
    }
  }
  else {
    if (piVar2 != (int *)0x0) {
      (**(code **)(*piVar2 + 4))(5);
      SetWindowPos(*(HWND *)(*(int *)((int)this + 0x248) + 0x18),(HWND)0x1,0,0,0,0,3);
    }
    if (*(int *)((int)this + 0xd4) < *(int *)((int)this + 0x11c) + 0xb4) {
      *(int *)((int)this + 0x11c) = *(int *)((int)this + 0xd4) + -0xb4;
    }
    if (*(int *)((int)this + 0xd8) < *(int *)((int)this + 0x120) + 0xf0) {
      *(int *)((int)this + 0x120) = *(int *)((int)this + 0xd8) + -0xf0;
    }
    pvVar3 = (void *)FUN_00618b70(0x14c);
    uStack_4 = 0;
    if (pvVar3 == (void *)0x0) {
      pvVar3 = (void *)0x0;
    }
    else {
      pvVar3 = FUN_00452090(pvVar3,uVar1,*(undefined4 *)((int)this + 0x11c),
                            *(undefined4 *)((int)this + 0x120),0xb4,0xf0,(int)this,0x43,param_1);
    }
    uStack_4 = 0xffffffff;
    *(void **)((int)this + 0x124) = pvVar3;
    FUN_005ffce0(pvVar3,0);
    SetWindowPos(*(HWND *)(*(int *)((int)this + 0x124) + 0x18),(HWND)0x1,0,0,0,0,3);
  }
  ExceptionList = pvStack_c;
  return;
}

