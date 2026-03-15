
undefined4 __fastcall FUN_0052bee0(int *param_1)

{
  bool bVar1;
  int iVar2;
  int iVar3;
  uint *puVar4;
  int unaff_EBP;
  int unaff_ESI;
  undefined4 uVar5;
  int *unaff_retaddr;
  void *local_14;
  undefined4 local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00644b90;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_004ece30(&local_10);
  local_4 = 0;
  FUN_004ece30(&local_14);
  local_4 = CONCAT31(local_4._1_3_,1);
  iVar2 = (**(code **)(*param_1 + 0xc))(&local_10);
  iVar3 = (**(code **)(*unaff_retaddr + 0xc))(&stack0xffffffe8);
  if ((iVar3 == 0) || (iVar2 == 0)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  iVar2 = FUN_004f7aa0(unaff_retaddr,1,unaff_retaddr);
  if ((iVar2 == 0) || (!bVar1)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  if (unaff_EBP != unaff_ESI) {
    puVar4 = FUN_004ece40((uint *)&stack0xffffffe8);
    if (puVar4 != (uint *)0x0) {
      puVar4 = FUN_004ece40((uint *)&stack0xffffffe4);
      if (puVar4 != (uint *)0x0) {
        iVar2 = FUN_004f7640(unaff_retaddr,1,unaff_retaddr);
        if ((iVar2 == 0) || (!bVar1)) {
          bVar1 = false;
        }
        else {
          bVar1 = true;
        }
      }
    }
  }
  iVar2 = FUN_004f74f0(unaff_retaddr,1,unaff_retaddr);
  if ((iVar2 == 0) || (!bVar1)) {
    uVar5 = 0;
  }
  else {
    uVar5 = 1;
  }
  pvStack_c = (void *)((uint)pvStack_c & 0xffffff00);
  FUN_00619730();
  pvStack_c = (void *)0xffffffff;
  FUN_00619730();
  ExceptionList = local_14;
  return uVar5;
}

