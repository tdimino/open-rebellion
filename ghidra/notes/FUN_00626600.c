
void __cdecl FUN_00626600(uint param_1,int *param_2,ushort *param_3)

{
  bool bVar1;
  undefined3 extraout_var;
  int iVar2;
  uint uVar3;
  uint local_58 [10];
  int local_30;
  int local_2c;
  uint local_20;
  
  param_3 = (ushort *)(uint)*param_3;
  switch(*param_2) {
  case 1:
  case 5:
    uVar3 = 8;
    break;
  case 2:
    uVar3 = 4;
    break;
  case 3:
    uVar3 = 0x11;
    break;
  case 4:
    uVar3 = 0x12;
    break;
  default:
    goto switchD_0062661f_caseD_6;
  case 7:
    *param_2 = 1;
    goto switchD_0062661f_caseD_6;
  case 8:
    uVar3 = 0x10;
  }
  bVar1 = FUN_0061f880(uVar3,(double *)(param_2 + 6),(uint)param_3);
  if (CONCAT31(extraout_var,bVar1) == 0) {
    if (((param_1 == 0x10) || (param_1 == 0x16)) || (param_1 == 0x1d)) {
      local_30 = param_2[4];
      local_20 = local_20 & 0xffffffe3 | 3;
      local_2c = param_2[5];
    }
    else {
      local_20 = local_20 & 0xfffffffe;
    }
    FUN_0061f540(local_58,(uint *)&param_3,(byte)uVar3,param_1,(uint *)(param_2 + 2),
                 (uint *)(param_2 + 6));
  }
switchD_0062661f_caseD_6:
  FUN_0061fee0();
  iVar2 = 0;
  if ((*param_2 != 8) && (DAT_006aff20 == 0)) {
    iVar2 = FUN_006158b0();
  }
  if (iVar2 == 0) {
    FUN_0061fc60(*param_2);
  }
  return;
}

