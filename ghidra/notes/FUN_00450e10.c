
void __thiscall FUN_00450e10(void *this,int *param_1,undefined4 param_2,int param_3,void *param_4)

{
  int *this_00;
  void *pvVar1;
  uint *puVar2;
  void *this_01;
  int iVar3;
  int iVar4;
  uint uVar5;
  undefined *puVar6;
  char *pcVar7;
  uint *puVar8;
  undefined1 uVar9;
  uint local_14;
  void *local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062fc25;
  local_c = ExceptionList;
  puVar8 = (uint *)0x0;
  ExceptionList = &local_c;
  pvVar1 = (void *)FUN_00618b70(0x74);
  this_00 = param_1;
  local_4 = 0;
  local_10 = pvVar1;
  if (pvVar1 == (void *)0x0) {
    this_01 = (void *)0x0;
  }
  else {
    puVar2 = FUN_004025b0(param_1,&local_14);
    local_4 = CONCAT31(local_4._1_3_,1);
    this_01 = FUN_004421d0(pvVar1,puVar2);
  }
  local_4 = 0;
  if (pvVar1 != (void *)0x0) {
    FUN_00619730();
  }
  local_4 = 0xffffffff;
  *(undefined4 *)((int)this_01 + 0x50) = 1;
  if (param_4 != (void *)0x0) {
    puVar8 = FUN_004025b0(this_00,(uint *)&param_1);
    local_4 = 2;
    puVar2 = FUN_0042c3b0(*(undefined4 *)((int)this + 0x138),puVar8,this_00,0,1);
    local_4 = 0xffffffff;
    FUN_00619730();
    *(undefined4 *)((int)this_01 + 0x30) = 0x46;
    puVar8 = puVar2;
    if (param_3 != 0) {
      pvVar1 = (void *)FUN_00618b70(0x28);
      local_4 = 3;
      param_4 = pvVar1;
      if (pvVar1 == (void *)0x0) {
        puVar8 = (uint *)0x0;
      }
      else {
        iVar3 = FUN_005fc0f0((int *)puVar2);
        iVar4 = FUN_005fc0e0((int *)puVar2);
        puVar8 = FUN_005fbda0(pvVar1,iVar4 + 0x14,iVar3,puVar8);
      }
      uVar9 = (undefined1)puVar2[2];
      local_4 = 0xffffffff;
      iVar3 = FUN_005fc0f0((int *)puVar8);
      uVar5 = FUN_005fc0e0((int *)puVar8);
      FUN_005fcfe0(puVar8,0,0,uVar5,iVar3,uVar9);
      FUN_005fd0f0(puVar2,puVar8,0x14,0);
      if (puVar2 != (uint *)0x0) {
        FUN_005fbfa0(puVar2);
        FUN_00618b60((undefined *)puVar2);
      }
      *(undefined4 *)((int)this_01 + 0x30) = 0x5a;
    }
    *(undefined4 *)((int)this_01 + 0x50) = 0;
  }
  *(undefined4 *)((int)this_01 + 0x38) = param_2;
  FUN_0060be60(this_01,(int)puVar8,0);
  puVar6 = FUN_004f62d0((int)this_00);
  pcVar7 = (char *)FUN_00583c40((int)puVar6);
  FUN_005f35e0((void *)((int)this_01 + 0x14),pcVar7);
  *(uint *)((int)this_01 + 0x3c) = *(uint *)((int)this_01 + 0x3c) | 4;
  FUN_005f59f0(*(void **)((int)this + 0x130),(int)this_01);
  ExceptionList = local_c;
  return;
}

