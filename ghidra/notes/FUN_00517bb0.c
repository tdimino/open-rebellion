
bool __cdecl FUN_00517bb0(int param_1,int param_2,void *param_3)

{
  bool bVar1;
  undefined3 extraout_var;
  
  bVar1 = true;
  if (*(int *)(param_1 + 0x18) == 0x109) {
    bVar1 = FUN_00517ca0(param_1,param_2,param_3);
    bVar1 = CONCAT31(extraout_var,bVar1) != 0;
  }
  return bVar1;
}

