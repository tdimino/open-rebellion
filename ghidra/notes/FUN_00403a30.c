
void __thiscall FUN_00403a30(void *this,uint *param_1)

{
  void *pvVar1;
  int *piVar2;
  undefined4 *this_00;
  uint *puVar3;
  uint uVar4;
  uint local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006295ab;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004ece30(&local_10);
  uVar4 = 0;
  local_4 = 0;
  FUN_005f4950(&local_14,0);
  puVar3 = param_1;
  local_4._0_1_ = 1;
  pvVar1 = FUN_004f5940(this,param_1);
  if (pvVar1 == (void *)0x0) {
    if (*(int *)((int)this + 0xc) != 0) {
      piVar2 = FUN_004f3110(*(int *)((int)this + 8),puVar3);
      if ((piVar2 != (int *)0x0) && ((*(byte *)(piVar2 + 0x14) & 8) == 0)) {
        param_1 = (uint *)FUN_00618b70(0x68);
        local_4._0_1_ = 2;
        if (param_1 == (uint *)0x0) {
          this_00 = (undefined4 *)0x0;
        }
        else {
          this_00 = FUN_00401d20(param_1,(undefined1 *)piVar2,*(undefined4 *)((int)this + 8),
                                 *(undefined4 *)((int)this + 0x10));
        }
        local_4._0_1_ = 1;
        if (this_00 != (undefined4 *)0x0) {
          if ((void *)piVar2[7] != (void *)0x0) {
            puVar3 = FUN_004025b0((void *)piVar2[7],(uint *)&param_1);
            local_4._0_1_ = 3;
            FUN_004f26d0(&local_10,puVar3);
            local_4._0_1_ = 1;
            FUN_00619730();
          }
          puVar3 = FUN_00403040(this_00,&param_1);
          local_4._0_1_ = 4;
          pvVar1 = FUN_004f5940(this,puVar3);
          local_4._0_1_ = 1;
          FUN_00619730();
          if (pvVar1 == (void *)0x0) {
            uVar4 = FUN_004f57b0(this,this_00);
            FUN_004021b0((int)this_00);
          }
          if (uVar4 == 0) {
            (**(code **)*this_00)(1);
          }
          else {
            FUN_004021b0((int)this_00);
            local_14 = this_00[0xc];
            if ((((local_14 & 2) != 0) && ((local_14 & 0x20000000) != 0)) && ((local_14 & 8) == 0))
            {
              *(int *)((int)this + 0x18) = *(int *)((int)this + 0x18) + -1;
            }
          }
        }
      }
    }
  }
  else {
    FUN_004021b0((int)pvVar1);
  }
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return;
}

