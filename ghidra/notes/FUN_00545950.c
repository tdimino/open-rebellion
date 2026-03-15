
bool FUN_00545950(uint *param_1,int param_2,void *param_3)

{
  bool bVar1;
  bool bVar2;
  undefined3 extraout_var;
  
  bVar2 = true;
  if ((*param_1 >> 0x18 < 0x90) || (0x97 < *param_1 >> 0x18)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  FUN_00619730();
  if ((bVar1) && (param_2 == 0)) {
    bVar2 = FUN_00548e90(param_1,param_3);
    bVar2 = CONCAT31(extraout_var,bVar2) != 0;
  }
  return bVar2;
}

