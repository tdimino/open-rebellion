
void __fastcall FUN_004e5710(int *param_1)

{
  int *piVar1;
  int iVar2;
  int unaff_EBX;
  int iVar3;
  undefined4 unaff_EBP;
  int unaff_ESI;
  uint unaff_EDI;
  int iVar4;
  undefined4 local_10;
  undefined4 local_c;
  undefined4 local_8;
  undefined4 local_4;
  
  iVar3 = 0;
  local_c = 0;
  local_8 = 0;
  local_10 = 0;
  local_4 = 0;
  (**(code **)(*param_1 + 0x18))(&local_c,&local_8,&local_10,&local_4);
  if (unaff_ESI != 0) {
    *(int *)(param_1[3] + 0x318) = unaff_ESI;
    *(undefined4 *)(param_1[3] + 0x31c) = unaff_EBP;
  }
  if (unaff_EDI != 0) {
    iVar4 = 2000000;
    for (piVar1 = (int *)thunk_FUN_005f5060(param_1[3] + 0xa8); piVar1 != (int *)0x0;
        piVar1 = (int *)piVar1[4]) {
      if (((((uint)piVar1[0xb] >> 0x18 == unaff_EDI) &&
           (iVar2 = (**(code **)(*piVar1 + 0x14))(), iVar2 == unaff_EBX)) &&
          ((piVar1[8] & 0x10000000U) == 0)) &&
         (iVar2 = (**(code **)(*piVar1 + 0x18))(), iVar2 < iVar4)) {
        iVar3 = piVar1[6];
        iVar4 = (**(code **)(*piVar1 + 0x18))();
      }
    }
    *(int *)(param_1[3] + 0x314) = iVar3;
  }
  return;
}

