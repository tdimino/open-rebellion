
void __thiscall FUN_004306e0(void *this,uint *param_1)

{
  undefined1 uVar1;
  void *pvVar2;
  int *piVar3;
  undefined4 *this_00;
  uint *puVar4;
  uint uVar5;
  void *local_10;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062ccb3;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  local_10 = this;
  FUN_004ece30(&local_10);
  puVar4 = param_1;
  local_4 = 0;
  pvVar2 = FUN_004f5940(this,param_1);
  if (pvVar2 == (void *)0x0) {
    if (*(int *)((int)this + 0xc) != 0) {
      piVar3 = FUN_004f2e20(*(int *)((int)this + 8),puVar4);
      if ((piVar3 != (int *)0x0) && ((*(byte *)(piVar3 + 0x14) & 8) == 0)) {
        param_1 = (uint *)FUN_00618b70(0xbc);
        local_4._0_1_ = 1;
        if (param_1 == (uint *)0x0) {
          this_00 = (undefined4 *)0x0;
        }
        else {
          this_00 = (undefined4 *)
                    FUN_00484230(param_1,piVar3,*(uint *)((int)this + 8),
                                 *(undefined4 *)((int)this + 0x14),*(void **)((int)this + 0x18),
                                 *(undefined4 *)((int)this + 0x1c));
        }
        local_4._0_1_ = 0;
        uVar1 = (undefined1)local_4;
        local_4._0_1_ = 0;
        if (this_00 != (undefined4 *)0x0) {
          if ((void *)piVar3[7] != (void *)0x0) {
            puVar4 = FUN_004025b0((void *)piVar3[7],(uint *)&param_1);
            local_4._0_1_ = 2;
            FUN_004f26d0(&local_10,puVar4);
            local_4._0_1_ = 0;
            FUN_00619730();
            uVar1 = (undefined1)local_4;
          }
          local_4._0_1_ = uVar1;
          uVar5 = FUN_00430450(this,this_00);
          if (uVar5 == 0) {
            (**(code **)*this_00)(1);
          }
          else {
            FUN_00484570((int)this_00);
            puVar4 = (uint *)FUN_00403040(this_00,&param_1);
            local_4._0_1_ = 3;
            FUN_00430120(*(void **)((int)this + 0x14),(uint *)&local_10,puVar4);
            local_4 = (uint)local_4._1_3_ << 8;
            FUN_00619730();
          }
        }
      }
    }
  }
  else {
    FUN_00484570((int)pvVar2);
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return;
}

