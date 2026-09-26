
void __thiscall FUN_004146e0(void *this,undefined4 param_1,int param_2,undefined4 param_3)

{
  int iVar1;
  int iVar2;
  undefined4 uVar3;
  undefined4 uVar4;
  undefined4 uVar5;
  HDC hdc;
  void *pvVar6;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_0062aa3b;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  if (*(int **)((int)this + 0xc0) != (int *)0x0) {
    ExceptionList = &pvStack_c;
    (**(code **)(**(int **)((int)this + 0xc0) + 8))();
  }
  if (*(int **)((int)this + 0xc4) != (int *)0x0) {
    (**(code **)(**(int **)((int)this + 0xc4) + 8))();
    DestroyWindow(*(HWND *)(*(int *)((int)this + 0xc4) + 0x18));
    if (*(undefined4 **)((int)this + 0xc4) != (undefined4 *)0x0) {
      (**(code **)**(undefined4 **)((int)this + 0xc4))(1);
    }
    *(undefined4 *)((int)this + 0xc4) = 0;
  }
  hdc = GetDC(*(HWND *)((int)this + 0x18));
  PatBlt(hdc,0,0,*(int *)((int)this + 0x38),*(int *)((int)this + 0x3c),0x42);
  FUN_00600230(hdc);
  ReleaseDC(*(HWND *)((int)this + 0x18),hdc);
  iVar1 = *(int *)((int)this + 0xa8);
  iVar2 = *(int *)((int)this + 0xac);
  uVar3 = *(undefined4 *)((int)this + 0x1c);
  uVar4 = *(undefined4 *)((int)this + 0xa0);
  uVar5 = *(undefined4 *)((int)this + 0xa4);
  pvVar6 = (void *)FUN_00618b70(0xb8);
  uStack_4 = 0;
  if (pvVar6 == (void *)0x0) {
    pvVar6 = (void *)0x0;
  }
  else {
    pvVar6 = FUN_004213e0(pvVar6,param_1,param_3,uVar3,uVar4,uVar5,iVar1,iVar2,this,param_2);
  }
  uStack_4 = 0xffffffff;
  *(void **)((int)this + 0xc4) = pvVar6;
  if (pvVar6 != (void *)0x0) {
    FUN_005ffce0(pvVar6,0);
    (**(code **)(**(int **)((int)this + 0xc4) + 4))(5);
    SetFocus(*(HWND *)(*(int *)((int)this + 0xc4) + 0x18));
  }
  ExceptionList = pvStack_c;
  return;
}

