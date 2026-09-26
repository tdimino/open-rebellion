
void __cdecl FUN_005f4b10(int *param_1,char *param_2)

{
  int iVar1;
  LPCSTR pCVar2;
  uint uVar3;
  uint uVar4;
  undefined4 local_18 [3];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00655738;
  local_c = ExceptionList;
  DAT_006ac240 = 0;
  ExceptionList = &local_c;
  FUN_005f35b0(local_18,param_2);
  local_4 = 0;
  iVar1 = FUN_00583c40((int)local_18);
  if (iVar1 == 0) {
    DAT_006ac240 = 1;
  }
  else {
    uVar3 = 0x82;
    uVar4 = DAT_0066e57c;
    pCVar2 = (LPCSTR)FUN_00583c40((int)local_18);
    FUN_006165b0(param_1,pCVar2,uVar3,uVar4);
    if (*(int *)(*(int *)(*(int *)(*param_1 + 4) + 4 + (int)param_1) + 0x4c) == -1) {
      DAT_006ac240 = 2;
    }
  }
  local_4 = 0xffffffff;
  FUN_005f2ff0(local_18);
  ExceptionList = local_c;
  return;
}

