
void __fastcall FUN_004e57e0(int *param_1)

{
  int *piVar1;
  int iVar2;
  int unaff_EBX;
  int unaff_EBP;
  int iVar3;
  undefined4 local_8;
  undefined4 local_4;
  
  iVar3 = 0;
  local_8 = 0;
  local_4 = 0;
  (**(code **)(*param_1 + 0x1c))(&local_8,&local_4);
  if (unaff_EBP != 0) {
    *(int *)(param_1[3] + 0x358) = unaff_EBP;
  }
  if (unaff_EBX != 0) {
    piVar1 = (int *)thunk_FUN_005f5060(param_1[3] + 0xec);
    while ((piVar1 != (int *)0x0 && (iVar3 == 0))) {
      iVar2 = (**(code **)(*piVar1 + 0xc))();
      if ((iVar2 == 2) && ((*(byte *)(piVar1 + 8) & 2) == 0)) {
        iVar3 = piVar1[6];
      }
      piVar1 = (int *)piVar1[4];
    }
    *(int *)(param_1[3] + 0x354) = iVar3;
  }
  return;
}

