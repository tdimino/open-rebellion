
uint __thiscall FUN_005306b0(void *this,void *param_1,undefined4 *param_2)

{
  void *this_00;
  undefined4 *puVar1;
  int iVar2;
  undefined4 *puVar3;
  uint *puVar4;
  undefined4 uVar5;
  uint uVar6;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puVar1 = param_2;
  this_00 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00645497;
  local_c = ExceptionList;
  uVar6 = 1;
  if (((*(byte *)((int)param_1 + 0x60) & 2) == 0) && (*(int *)((int)param_1 + 0x58) == 1)) {
    param_1 = (void *)0x0;
    ExceptionList = &local_c;
    iVar2 = FUN_0052fb30(this,(uint *)&param_1,param_2);
    uVar6 = 0;
    if (iVar2 != 0) {
      if (param_1 == (void *)0x0) {
        puVar3 = (undefined4 *)FUN_00618b70(0x18);
        if (puVar3 == (undefined4 *)0x0) {
          FUN_00619730();
        }
        local_4 = 0;
        param_2 = puVar3;
        if (puVar3 == (undefined4 *)0x0) {
          puVar3 = (undefined4 *)0x0;
        }
        else {
          puVar3 = FUN_00583c70(puVar3);
        }
        local_4 = 0xffffffff;
        uVar6 = 0;
        if (puVar3 != (undefined4 *)0x0) {
          if (puVar3[3] == 0) {
            if (puVar3 != (undefined4 *)0x0) {
              (**(code **)*puVar3)(1);
            }
            uVar6 = 0;
          }
          else {
            local_4 = 0xffffffff;
            puVar4 = FUN_004025b0(this_00,(uint *)&param_2);
            local_4 = 1;
            FUN_004f26d0(puVar3 + 4,puVar4);
            local_4 = 0xffffffff;
            FUN_00619730();
            uVar5 = FUN_0053a6e0((int)this_00);
            puVar3[5] = uVar5;
            uVar6 = FUN_0052fbb0(this,(int)puVar3,puVar1);
          }
        }
      }
      else {
        uVar6 = FUN_0053a860(this_00,1,puVar1);
      }
    }
  }
  ExceptionList = local_c;
  return uVar6;
}

