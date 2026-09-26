
void __cdecl
FUN_00559ce0(int param_1,int param_2,int param_3,int param_4,int param_5,int param_6,int param_7,
            undefined4 *param_8,undefined4 *param_9)

{
  undefined4 *puVar1;
  undefined4 *puVar2;
  bool bVar3;
  int iVar4;
  int iVar5;
  int iVar6;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  int iVar7;
  
  puVar2 = param_9;
  puVar1 = param_8;
  *param_8 = 0;
  *param_9 = 0;
  iVar4 = FUN_0053e290(DAT_006bb418);
  iVar4 = iVar4 + DAT_006bb39c;
  iVar5 = FUN_0053e290(DAT_006bb418);
  iVar5 = iVar5 + DAT_006bb39c;
  if ((param_3 == 0) || (iVar7 = DAT_006bb3dc, param_1 != 2)) {
    iVar7 = 1;
  }
  iVar6 = FUN_0055a050(param_2);
  iVar4 = iVar4 + iVar5 + (iVar6 - iVar7 * param_4) + ((param_7 + param_6) - param_5);
  bVar3 = FUN_0053e240(0x28,iVar4,&param_8);
  if ((CONCAT31(extraout_var,bVar3) != 0) && (*puVar1 = param_8, CONCAT31(extraout_var,bVar3) != 0))
  {
    bVar3 = FUN_0053e240(0x29,iVar4,&param_8);
    if (CONCAT31(extraout_var_00,bVar3) != 0) {
      *puVar2 = param_8;
    }
  }
  return;
}

