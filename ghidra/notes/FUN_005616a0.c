
bool __thiscall
FUN_005616a0(void *this,undefined4 param_1,uint *param_2,uint *param_3,void *param_4)

{
  bool bVar1;
  bool bVar2;
  uint uVar3;
  int iVar4;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  undefined3 extraout_var_01;
  
  bVar1 = true;
  if (((*param_2 ^ *param_3) & 3) != 0) {
    uVar3 = *param_2 & 3;
    if (uVar3 != 3) {
      iVar4 = FUN_0053ec90(uVar3,0);
      bVar1 = FUN_00561910(iVar4,*param_2 & 3,param_4);
      bVar1 = CONCAT31(extraout_var,bVar1) != 0;
    }
    if ((*param_3 & 3) != 3) {
      iVar4 = FUN_0053ec90(*param_3 & 3,1);
      bVar2 = FUN_00561910(iVar4,*param_3 & 3,param_4);
      if ((CONCAT31(extraout_var_00,bVar2) == 0) || (!bVar1)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
    }
    bVar2 = FUN_005617b0(this,param_4);
    if ((CONCAT31(extraout_var_01,bVar2) != 0) && (bVar1)) {
      return true;
    }
    bVar1 = false;
  }
  return bVar1;
}

