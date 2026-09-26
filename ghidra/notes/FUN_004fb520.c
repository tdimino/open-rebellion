
undefined4 __thiscall FUN_004fb520(void *this,undefined4 param_1,void *param_2)

{
  int iVar1;
  int *piVar2;
  undefined4 uVar3;
  bool bVar4;
  int unaff_retaddr;
  int iStack_14;
  void *pvStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_006400c8;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  iVar1 = (**(code **)(*(int *)this + 0x88))(param_2);
  bVar4 = iVar1 != 0;
  FUN_004fcd00(&stack0xffffffd0,this,3);
  puStack_8 = (undefined1 *)0x0;
  FUN_00513120((int)&stack0xffffffd0);
  while (iStack_14 != 0) {
    piVar2 = (int *)FUN_0052bed0((int)&stack0xffffffd0);
    iVar1 = (**(code **)(*piVar2 + 0xb4))(1,param_2);
    if ((iVar1 == 0) || (!bVar4)) {
      bVar4 = false;
    }
    else {
      bVar4 = true;
    }
    FUN_005130d0((int)&stack0xffffffd0);
  }
  if (unaff_retaddr != 0) {
    iVar1 = FUN_004f7640(this,0,param_2);
    if ((iVar1 == 0) || (!bVar4)) {
      bVar4 = false;
    }
    else {
      bVar4 = true;
    }
    iVar1 = FUN_004f7870(this,0,param_2);
    if ((iVar1 == 0) || (!bVar4)) {
      bVar4 = false;
    }
    else {
      bVar4 = true;
    }
    iVar1 = FUN_004f78e0(this,0,param_2);
    if ((iVar1 == 0) || (!bVar4)) {
      bVar4 = false;
    }
    else {
      bVar4 = true;
    }
  }
  iVar1 = FUN_004f8240(this,param_2);
  if ((iVar1 == 0) || (!bVar4)) {
    bVar4 = false;
  }
  else {
    bVar4 = true;
  }
  iVar1 = FUN_004f81c0(this,param_2);
  if ((iVar1 == 0) || (!bVar4)) {
    bVar4 = false;
  }
  else {
    bVar4 = true;
  }
  iVar1 = FUN_004f80e0(this,param_2);
  if ((iVar1 == 0) || (!bVar4)) {
    bVar4 = false;
  }
  else {
    bVar4 = true;
  }
  iVar1 = FUN_004f82d0(this,param_2);
  if ((iVar1 == 0) || (!bVar4)) {
    uVar3 = 0;
  }
  else {
    uVar3 = 1;
  }
  puStack_8 = (undefined1 *)0xffffffff;
  FUN_004fcd80((undefined4 *)&stack0xffffffd0);
  ExceptionList = pvStack_10;
  return uVar3;
}

