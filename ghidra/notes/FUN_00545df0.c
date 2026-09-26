
bool __thiscall FUN_00545df0(void *this,uint *param_1,int param_2,void *param_3)

{
  bool bVar1;
  bool bVar2;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  int iVar3;
  
  bVar1 = true;
  if ((*param_1 >> 0x18 < 0x30) || (0x3f < *param_1 >> 0x18)) {
    bVar2 = false;
  }
  else {
    bVar2 = true;
  }
  FUN_00619730();
  if (bVar2) {
    if (param_2 != 0) {
      bVar1 = FUN_00548da0(param_1,param_3);
      bVar1 = CONCAT31(extraout_var,bVar1) != 0;
    }
    bVar2 = FUN_005438a0((int *)param_1);
    if (CONCAT31(extraout_var_00,bVar2) != 0) {
      iVar3 = FUN_00549910(this,param_3);
      if ((iVar3 != 0) && (bVar1)) {
        return true;
      }
      bVar1 = false;
    }
  }
  return bVar1;
}

