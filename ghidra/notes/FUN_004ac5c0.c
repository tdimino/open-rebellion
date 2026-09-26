
undefined4 __thiscall FUN_004ac5c0(void *this,int param_1,LONG param_2,int param_3)

{
  HWND pHVar1;
  int iVar2;
  int iVar3;
  int *piVar4;
  byte bVar5;
  void *pvVar6;
  undefined4 *puVar7;
  undefined4 auStack_14 [2];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  iVar3 = param_3;
  iVar2 = DAT_006be3b8;
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_006385a8;
  local_c = ExceptionList;
  if (((DAT_006be3b8 != 0) && (param_3 != 0)) &&
     (ExceptionList = &local_c, pHVar1 = GetCapture(), pHVar1 == (HWND)0x0)) {
    pvVar6 = *(void **)(iVar2 + 0xc0);
    iVar2 = FUN_00401060();
    piVar4 = (int *)((int)this + 0x11c);
    (**(code **)(*piVar4 + 4))();
    (**(code **)(*(int *)this + 0x58))(piVar4,iVar3);
    if (pvVar6 != (void *)0x0) {
      MapWindowPoints(*(HWND *)(iVar3 + 0x18),*(HWND *)((int)pvVar6 + 0x18),(LPPOINT)&param_1,1);
    }
    FUN_005205a0(auStack_14);
    puVar7 = auStack_14;
    uStack_4 = 0;
    iVar3 = FUN_00401060();
    FUN_0041dcc0(iVar3,(uint)piVar4,puVar7);
    for (iVar3 = thunk_FUN_005f5060((int)auStack_14); iVar3 != 0; iVar3 = *(int *)(iVar3 + 0x10)) {
      bVar5 = *(int *)(iVar3 + 0x1c) == 0;
      if (*(int *)(iVar3 + 0x20) != 0) {
        bVar5 = bVar5 | 2;
      }
      FUN_00442590(*(uint *)(iVar3 + 0x18),(void *)0x7,(uint)bVar5,param_1,param_2,pvVar6,
                   (-(uint)(iVar2 != 1) & 0xfe01) + 0x20000ff,0x2ffffff,0x2808080);
    }
    FUN_006073d0(this);
    piVar4 = (int *)FUN_006037f0(7);
    FUN_00442380(param_1,param_2,piVar4,(int)pvVar6,this);
    uStack_4 = 0xffffffff;
    FUN_005205e0(auStack_14);
  }
  ExceptionList = local_c;
  return 0;
}

