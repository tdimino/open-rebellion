
uint __thiscall FUN_00567c90(void *this,int param_1,uint param_2,undefined4 *param_3)

{
  short sVar1;
  uint uVar2;
  undefined2 extraout_var;
  int iVar3;
  int local_30;
  undefined4 local_2c [4];
  undefined4 local_1c [4];
  void *local_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064b280;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *param_3 = 1;
  FUN_0052d720(local_1c);
  local_4 = 0;
  local_30 = 0;
  uVar2 = FUN_00567880(this,param_1,param_2,0,0,0,0,local_1c,&local_30);
  for (iVar3 = 1; (uVar2 != 0 && (iVar3 <= local_30)); iVar3 = iVar3 + 1) {
    FUN_0052d720(local_2c);
    local_4 = CONCAT31(local_4._1_3_,1);
    uVar2 = FUN_00567880(this,param_1,param_2,iVar3,0,1,0,local_2c,&local_30);
    if (uVar2 != 0) {
      sVar1 = FUN_004f4290((int)local_2c);
      if ((CONCAT22(extraout_var,sVar1) != 0) && (CONCAT22(extraout_var,sVar1) != 1)) {
        *param_3 = 0;
      }
    }
    local_4 = local_4 & 0xffffff00;
    FUN_0052d760(local_2c);
  }
  local_4 = 0xffffffff;
  FUN_0052d760(local_1c);
  ExceptionList = local_c;
  return uVar2;
}

