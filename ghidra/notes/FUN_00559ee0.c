
void __cdecl FUN_00559ee0(int *param_1,int *param_2,int param_3,int param_4)

{
  bool bVar1;
  int iVar2;
  undefined3 extraout_var;
  undefined4 *puVar3;
  undefined4 local_4;
  
  puVar3 = &local_4;
  iVar2 = FUN_0053e2e0();
  bVar1 = FUN_0053e240(0x2b,iVar2,puVar3);
  if (CONCAT31(extraout_var,bVar1) != 0) {
    switch(local_4) {
    case 1:
      if ((param_3 != 0) && (DAT_006bb4b4 < *param_1)) {
        *param_1 = *param_1 + -1;
        return;
      }
      break;
    case 2:
      if ((param_4 != 0) && (DAT_006bb3e0 < *param_2)) {
        *param_2 = *param_2 + -1;
        return;
      }
      break;
    case 3:
      iVar2 = *param_1;
      if ((iVar2 < DAT_006bb40c) && (iVar2 < *param_2)) {
        *param_1 = iVar2 + 1;
        return;
      }
      break;
    case 4:
      if (*param_2 < DAT_006bb488) {
        *param_2 = *param_2 + 1;
      }
    }
  }
  return;
}

