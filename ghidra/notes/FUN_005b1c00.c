
void __thiscall FUN_005b1c00(void *this,uint *param_1)

{
  uint uVar1;
  void *pvVar2;
  int *piVar3;
  int iVar4;
  int *piVar5;
  uint *puVar6;
  uint local_68 [4];
  undefined4 local_58;
  undefined4 local_54;
  undefined4 local_50;
  undefined4 local_4c;
  undefined4 local_48;
  undefined4 local_44;
  undefined4 local_40;
  undefined4 local_3c;
  undefined4 local_38;
  undefined4 local_34;
  undefined4 local_30;
  undefined4 local_2c;
  undefined4 local_28;
  undefined4 local_24;
  undefined4 local_20;
  int local_1c;
  int local_18;
  undefined4 local_14;
  int local_10;
  undefined4 local_c;
  undefined4 local_8;
  int local_4;
  
  FUN_005b1dc0(local_68);
  local_58 = *(undefined4 *)((int)this + 0x36c);
  local_54 = *(undefined4 *)((int)this + 0x370);
  local_50 = *(undefined4 *)((int)this + 0x374);
  local_4c = *(undefined4 *)((int)this + 0x378);
  local_48 = *(undefined4 *)((int)this + 0x37c);
  local_44 = *(undefined4 *)((int)this + 0x3b8);
  local_40 = *(undefined4 *)((int)this + 0x3bc);
  local_3c = *(undefined4 *)((int)this + 0x3c0);
  local_38 = *(undefined4 *)((int)this + 0x3c8);
  local_34 = *(undefined4 *)((int)this + 0x3cc);
  local_30 = *(undefined4 *)((int)this + 0x3e4);
  local_2c = *(undefined4 *)((int)this + 1000);
  local_68[0] = *(uint *)((int)this + 0x170);
  local_28 = *(undefined4 *)((int)this + 0x3ec);
  local_68[1] = *(undefined4 *)((int)this + 0xec);
  local_24 = *(undefined4 *)((int)this + 0x3f8);
  local_20 = *(undefined4 *)((int)this + 0x3fc);
  local_68[2] = *(uint *)((int)this + 0x174);
  local_68[3] = *(uint *)((int)this + 0x178);
  local_1c = 0;
  local_18 = 0;
  local_14 = 0;
  local_10 = 0;
  local_4 = 0;
  local_c = 0;
  local_8 = 0;
  uVar1 = *(uint *)((int)this + 0x178);
  if (*(int *)((int)this + 0x124) != 0) {
    uVar1 = *(uint *)((int)this + 0x174);
  }
  if (uVar1 != 0) {
    pvVar2 = (void *)FUN_00596bd0();
    piVar3 = (int *)FUN_005a6480(pvVar2,uVar1);
    if (piVar3 != (int *)0x0) {
      local_14 = FUN_005cb980((int)piVar3);
      iVar4 = (**(code **)(*piVar3 + 4))();
      if (iVar4 == 3) {
        pvVar2 = (void *)FUN_0059ed70();
        local_18 = FUN_0059f640(pvVar2,(int)this);
      }
      local_10 = piVar3[0x1a];
      local_1c = piVar3[0x17];
      local_4 = piVar3[0xe];
      iVar4 = (**(code **)(piVar3[0x26] + 0x10))();
      if (iVar4 != 0) {
        piVar5 = (int *)(**(code **)(piVar3[0x26] + 0x10))();
        local_c = (**(code **)(*piVar5 + 4))();
      }
      iVar4 = (**(code **)(piVar3[0x20] + 0xc))();
      if (iVar4 != 0) {
        iVar4 = (**(code **)(piVar3[0x20] + 0xc))();
        local_8 = *(undefined4 *)(iVar4 + 0x18);
      }
    }
  }
  puVar6 = local_68;
  for (iVar4 = 0x1a; iVar4 != 0; iVar4 = iVar4 + -1) {
    *param_1 = *puVar6;
    puVar6 = puVar6 + 1;
    param_1 = param_1 + 1;
  }
  return;
}

