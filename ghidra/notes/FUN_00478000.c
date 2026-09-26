
void __fastcall FUN_00478000(int param_1)

{
  uint uVar1;
  int iVar2;
  int *piVar3;
  uint uVar4;
  int iVar5;
  void *unaff_EBX;
  int iVar6;
  int unaff_ESI;
  uint unaff_EDI;
  undefined1 local_18 [4];
  undefined4 local_14;
  undefined1 local_10 [4];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00633168;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_005f4950(local_10,0);
  piVar3 = (int *)(param_1 + 0xbc);
  *piVar3 = 0;
  *(undefined4 *)(param_1 + 0xc0) = 0;
  *(undefined4 *)(param_1 + 0xc4) = 0;
  local_4 = 0;
  (**(code **)(**(int **)(param_1 + 0x5c) + 0x18))
            (piVar3,(undefined4 *)(param_1 + 0xc0),local_18,&local_14);
  if (*piVar3 != 0) {
    *(uint *)(param_1 + 100) = *(uint *)(param_1 + 100) | 0x100;
  }
  iVar6 = 2000000000;
  for (iVar2 = thunk_FUN_005f5060(param_1 + 0xb0); iVar2 != 0; iVar2 = *(int *)(iVar2 + 0x10)) {
    piVar3 = (int *)FUN_005f5500((void *)(*(int *)(param_1 + 0x58) + 0xa8),*(uint *)(iVar2 + 0x18));
    if (piVar3 != (int *)0x0) {
      uVar1 = *(uint *)(param_1 + 100);
      uVar4 = uVar1 | 0x20000000;
      *(uint *)(param_1 + 100) = uVar4;
      if (((byte)piVar3[8] & 3) != 3) {
        *(uint *)(param_1 + 100) = CONCAT22((short)(uVar4 >> 0x10),(short)uVar1) | 0x400;
      }
      if ((piVar3[8] & 0x100U) != 0) {
        *(uint *)(param_1 + 100) = *(uint *)(param_1 + 100) | 0x80000000;
      }
      if ((((unaff_EDI != 0) && ((uint)piVar3[0xb] >> 0x18 == unaff_EDI)) &&
          (iVar5 = (**(code **)(*piVar3 + 0x14))(), iVar5 == unaff_ESI)) &&
         (((piVar3[8] & 0x10000000U) == 0 &&
          (iVar5 = (**(code **)(*piVar3 + 0x18))(), iVar5 < iVar6)))) {
        *(int *)(param_1 + 0xc4) = piVar3[6];
        iVar6 = (**(code **)(*piVar3 + 0x18))();
      }
    }
  }
  local_14 = 0xffffffff;
  FUN_00619730();
  ExceptionList = unaff_EBX;
  return;
}

