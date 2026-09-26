
void __cdecl FUN_00559db0(undefined4 *param_1)

{
  undefined4 *puVar1;
  bool bVar2;
  int iVar3;
  undefined3 extraout_var;
  
  puVar1 = param_1;
  *param_1 = 0;
  iVar3 = FUN_0053e290(DAT_006bb3ac);
  bVar2 = FUN_0053e240(0x2a,iVar3 + DAT_006bb3b0,&param_1);
  if (CONCAT31(extraout_var,bVar2) != 0) {
    *puVar1 = param_1;
  }
  return;
}

