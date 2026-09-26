
bool __cdecl FUN_00515390(uint *param_1,uint *param_2,void *param_3)

{
  int iVar1;
  uint uVar2;
  bool bVar3;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00642698;
  local_c = ExceptionList;
  local_14 = 1;
  local_10 = 1;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_3,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  iVar1 = thunk_FUN_00506e60();
  bVar3 = iVar1 != 0;
  if (iVar1 != 0) {
    uVar2 = FUN_00555920(param_1,param_2,param_3);
    if ((uVar2 != 0) && (bVar3)) {
      ExceptionList = local_c;
      return true;
    }
    bVar3 = false;
  }
  ExceptionList = local_c;
  return bVar3;
}

