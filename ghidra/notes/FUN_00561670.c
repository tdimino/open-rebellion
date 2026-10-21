
bool __thiscall FUN_00561670(void *this,int param_1,uint *param_2,uint *param_3,void *param_4)

{
  bool bVar1;
  undefined3 extraout_var;
  
  bVar1 = true;
  if (*(int *)(param_1 + 0x18) == 0x109) {
    bVar1 = FUN_005616a0(this,param_1,param_2,param_3,param_4);
    bVar1 = CONCAT31(extraout_var,bVar1) != 0;
  }
  return bVar1;
}

