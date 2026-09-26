
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __thiscall FUN_004eabb0(void *this,uint *param_1,float *param_2)

{
  int iVar1;
  void *pvVar2;
  int *piVar3;
  uint *puVar4;
  uint **ppuVar5;
  float local_3c;
  int local_38;
  int local_34;
  undefined4 local_2c [7];
  int local_10;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  puVar4 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063e8d0;
  local_c = ExceptionList;
  iVar1 = *(int *)((int)this + 0xc);
  local_3c = 0.9;
  local_34 = 0;
  local_38 = 0;
  ExceptionList = &local_c;
  FUN_004f26d0(&stack0xffffffb0,param_1);
  pvVar2 = FUN_00403d30((void *)(iVar1 + 0x2c));
  if (((pvVar2 != (void *)0x0) && ((*(byte *)((int)pvVar2 + 0x30) & 1) != 0)) &&
     (0 < *(int *)((int)pvVar2 + 0xac))) {
    if ((*(uint *)((int)pvVar2 + 0x28) & 0x80000000) == 0) {
      piVar3 = FUN_004f3220(*(int *)((int)this + 4),puVar4);
      if (piVar3 != (int *)0x0) {
        FUN_004ffe70(local_2c,piVar3,3);
        local_4 = 0;
        FUN_00513120((int)local_2c);
        while (local_10 != 0) {
          ppuVar5 = &param_1;
          pvVar2 = (void *)FUN_0052bed0((int)local_2c);
          puVar4 = FUN_004025b0(pvVar2,(uint *)ppuVar5);
          iVar1 = *(int *)((int)this + 0xc);
          local_4._0_1_ = 1;
          FUN_004f26d0(&stack0xffffffb0,puVar4);
          pvVar2 = FUN_00419e40((void *)(iVar1 + 0x78));
          local_4 = (uint)local_4._1_3_ << 8;
          FUN_00619730();
          if (pvVar2 != (void *)0x0) {
            if ((*(byte *)((int)pvVar2 + 0x38) & 1) == 0) {
              local_38 = local_38 + *(int *)((int)pvVar2 + 0x58);
            }
            else {
              local_34 = local_34 + *(int *)((int)pvVar2 + 0x58);
            }
          }
          FUN_005130d0((int)local_2c);
        }
        local_3c = (float)local_34;
        if (0 < local_38) {
          local_3c = local_3c / (float)local_38;
        }
        if (_DAT_0065ca04 < local_3c) {
          local_3c = 0.9;
        }
        local_4 = 0xffffffff;
        FUN_004fff70(local_2c);
      }
    }
    else {
      local_3c = 0.0;
    }
  }
  *param_2 = local_3c;
  ExceptionList = local_c;
  return;
}

