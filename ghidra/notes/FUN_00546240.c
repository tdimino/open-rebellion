
uint FUN_00546240(uint *param_1,int param_2,undefined4 param_3)

{
  undefined4 uVar1;
  void *this;
  uint *puVar2;
  int iVar3;
  int *this_00;
  uint uVar4;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006476a8;
  local_c = ExceptionList;
  uVar4 = 1;
  if (param_2 != 0) {
    ExceptionList = &local_c;
    this = (void *)FUN_00505d40(param_1);
    uVar4 = (uint)(this != (void *)0x0);
    if (this != (void *)0x0) {
      puVar2 = (uint *)FUN_0042d170(this,&param_2);
      local_4 = 0;
      iVar3 = FUN_004ece60(puVar2);
      local_4 = 0xffffffff;
      FUN_00619730();
      if (((iVar3 != 0) && ((*(byte *)((int)this + 0x50) & 8) != 0)) &&
         ((*(byte *)((int)this + 0x78) & 4) != 0)) {
        puVar2 = (uint *)FUN_0042d170(this,&param_2);
        local_4 = 1;
        this_00 = (int *)FUN_00505970(puVar2);
        local_4 = 0xffffffff;
        FUN_00619730();
        uVar1 = param_3;
        if (((uVar4 != 0) && (uVar4 = (uint)(this_00 != (int *)0x0), uVar4 != 0)) &&
           (this_00 != (int *)0x0)) {
          puVar2 = FUN_004025b0(this,(uint *)&param_1);
          local_4 = 2;
          (**(code **)(*this_00 + 0x1d0))(puVar2,0,uVar1);
          local_4 = 0xffffffff;
          FUN_00619730();
          uVar4 = FUN_00522480(this_00,uVar1);
        }
      }
    }
  }
  ExceptionList = local_c;
  return uVar4;
}

