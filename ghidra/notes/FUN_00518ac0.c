
bool __cdecl FUN_00518ac0(void *param_1,uint *param_2,void *param_3)

{
  bool bVar1;
  undefined3 extraout_var;
  int iVar2;
  int local_4;
  
  local_4 = 0;
  bVar1 = FUN_004f6b50(param_1,&local_4);
  bVar1 = CONCAT31(extraout_var,bVar1) != 0;
  if (local_4 != 0) {
    iVar2 = FUN_00518b20(local_4,param_2,param_3);
    if ((iVar2 != 0) && (bVar1)) {
      return true;
    }
    bVar1 = false;
  }
  return bVar1;
}

