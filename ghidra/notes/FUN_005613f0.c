
bool __thiscall FUN_005613f0(void *this,uint *param_1,uint *param_2,uint *param_3,void *param_4)

{
  bool bVar1;
  bool bVar2;
  int iVar3;
  undefined3 extraout_var;
  
  bVar2 = true;
  if ((*param_1 >> 0x18 < 0x90) || (0x97 < *param_1 >> 0x18)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  FUN_00619730();
  if (bVar1) {
    iVar3 = FUN_00505f60(param_1);
    if (iVar3 != 0) {
      bVar2 = FUN_00561670(this,iVar3,param_2,param_3,param_4);
      bVar2 = CONCAT31(extraout_var,bVar2) != 0;
    }
  }
  return bVar2;
}

