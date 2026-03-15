
void FUN_004c3930(int *param_1)

{
  bool bVar1;
  int *piVar2;
  int *piVar3;
  uint *puVar4;
  int *piVar5;
  int iVar6;
  int unaff_EBX;
  void *pvStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  piVar2 = param_1;
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_0063a628;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  puVar4 = (uint *)(**(code **)(*param_1 + 0x30))(&param_1);
  bVar1 = true;
  puStack_8 = (undefined1 *)0x0;
  piVar5 = FUN_004f2e20(1,puVar4);
  puStack_8 = (undefined1 *)0xffffffff;
  FUN_00619730();
  piVar3 = param_1;
  if ((*param_1 != 0x14) || (param_1[1] != 1)) {
    bVar1 = false;
  }
  FUN_00619730();
  if (bVar1) {
    if (piVar5 != (int *)0x0) {
      iVar6 = (**(code **)(*piVar5 + 0x26c))();
      if (iVar6 != 0) {
        *(int *)(*(int *)(unaff_EBX + 0x168) + 0x20) = DAT_006b28cc + 5;
        ExceptionList = pvStack_10;
        return;
      }
LAB_004c3a9a:
      *(int *)(*(int *)(unaff_EBX + 0x168) + 0x1c) = DAT_006b28cc + 5;
      ExceptionList = pvStack_10;
      return;
    }
    puVar4 = (uint *)(**(code **)(*piVar2 + 0x30))(&stack0x00000000);
    bVar1 = true;
    puStack_8 = (undefined1 *)0x1;
    if ((*puVar4 >> 0x18 < 8) || (0xf < *puVar4 >> 0x18)) {
      bVar1 = false;
    }
    FUN_00619730();
  }
  else {
    if ((*piVar3 == 0x14) && (piVar3[1] == 2)) {
      bVar1 = true;
    }
    else {
      bVar1 = false;
    }
    FUN_00619730();
    if (!bVar1) goto LAB_004c3b1a;
    if (piVar5 != (int *)0x0) {
      iVar6 = (**(code **)(*piVar5 + 0x270))();
      if (iVar6 != 0) {
        *(int *)(*(int *)(unaff_EBX + 0x168) + 0x20) = DAT_006b28cc + 5;
        ExceptionList = pvStack_10;
        return;
      }
      goto LAB_004c3a9a;
    }
    puVar4 = (uint *)(**(code **)(*piVar2 + 0x30))(&stack0x00000000);
    puStack_8 = (undefined1 *)0x2;
    if ((*puVar4 >> 0x18 < 8) || (0xf < *puVar4 >> 0x18)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
  }
  puStack_8 = (undefined1 *)0xffffffff;
  FUN_00619730();
  if (bVar1) {
    *(int *)(*(int *)(unaff_EBX + 0x168) + 0x24) = DAT_006b28cc + 5;
    ExceptionList = pvStack_10;
    return;
  }
LAB_004c3b1a:
  thunk_FUN_004c4390(unaff_EBX);
  ExceptionList = pvStack_10;
  return;
}

