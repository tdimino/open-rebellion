
uint __thiscall FUN_004fcb50(void *this,void *param_1)

{
  bool bVar1;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  undefined3 extraout_var_01;
  int *this_00;
  undefined3 extraout_var_02;
  int iVar2;
  int iVar3;
  uint uVar4;
  uint local_14 [2];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00640258;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  bVar1 = FUN_0053a010((int)this);
  uVar4 = CONCAT31(extraout_var,bVar1);
  bVar1 = FUN_005406d0(this);
  if ((CONCAT31(extraout_var_00,bVar1) != 0) && (uVar4 != 0)) {
    iVar3 = *(int *)((int)this + 0x1c);
    bVar1 = FUN_00539ff0(iVar3);
    uVar4 = 0;
    if (CONCAT31(extraout_var_01,bVar1) != 0) {
      local_14[0] = 0xf2;
      local_14[1] = 0xf3;
      local_4 = 0;
      this_00 = FUN_0053f030(local_14,*(uint *)((int)this + 0x24) >> 4 & 3,1);
      local_4 = 0xffffffff;
      FUN_00619730();
      uVar4 = 0;
      if (this_00 != (int *)0x0) {
        bVar1 = FUN_004fc600(this_00,this,param_1);
        uVar4 = (uint)(CONCAT31(extraout_var_02,bVar1) != 0);
        iVar2 = FUN_004fd3d0();
        if (iVar2 != 0) {
          iVar2 = FUN_004fd600(param_1,*(uint *)((int)this + 0x24) >> 4 & 3);
          if (iVar2 != 0) {
            iVar3 = (**(code **)(*(int *)this + 0x15c))(iVar3,this_00,param_1);
            if ((iVar3 == 0) || (uVar4 == 0)) {
              uVar4 = 0;
            }
            else {
              uVar4 = 1;
            }
          }
        }
      }
    }
  }
  ExceptionList = local_c;
  return uVar4;
}

