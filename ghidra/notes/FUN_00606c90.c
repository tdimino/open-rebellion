
undefined4 __thiscall
FUN_00606c90(void *this,undefined4 param_1,undefined4 param_2,uint param_3,uint param_4,
            undefined4 param_5,UINT param_6,uint param_7,undefined4 param_8,undefined4 param_9,
            undefined4 param_10,int param_11,int param_12)

{
  undefined4 uVar1;
  void *pvVar2;
  int *piVar3;
  uint *puVar4;
  int iVar5;
  int iVar6;
  int *this_00;
  void *pvVar7;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006565c6;
  local_c = ExceptionList;
  this_00 = (int *)0x0;
  ExceptionList = &local_c;
  uVar1 = FUN_006037f0(*(int *)((int)this + 0x98));
  pvVar2 = (void *)FUN_00618b70(0x28);
  local_4 = 0;
  if (pvVar2 == (void *)0x0) {
    piVar3 = (int *)0x0;
  }
  else {
    piVar3 = FUN_005fbd20(pvVar2,uVar1,param_3,2);
  }
  local_4 = 0xffffffff;
  if (piVar3 != (int *)0x0) {
    pvVar2 = (void *)FUN_00618b70(0x114);
    local_4 = 1;
    if (pvVar2 == (void *)0x0) {
      this_00 = (int *)0x0;
    }
    else {
      puVar4 = (uint *)FUN_006073d0(this);
      pvVar7 = this;
      iVar5 = FUN_005fc0f0(piVar3);
      iVar6 = FUN_005fc0e0(piVar3);
      this_00 = FUN_00602150(pvVar2,uVar1,param_1,param_2,iVar6,iVar5,pvVar7,param_5,param_3,param_4
                             ,puVar4,param_11,param_12);
    }
    local_4 = 0xffffffff;
    FUN_005fbfa0(piVar3);
    FUN_00618b60((undefined *)piVar3);
  }
  if (this_00 == (int *)0x0) {
    uVar1 = 0;
  }
  else {
    FUN_005ffce0(this_00,0);
    if (param_6 != 0) {
      FUN_006009f0(this,this_00,param_6);
    }
    if (param_7 != 0) {
      FUN_006025a0(this_00,param_7);
    }
    (**(code **)(*this_00 + 0x18))(param_8);
    FUN_00603010(this_00,0,param_9);
    FUN_00603010(this_00,1,param_10);
    FUN_00603010(this_00,4,param_10);
    FUN_005f4f10((void *)((int)this + 0x6c),(int)this_00);
    uVar1 = 1;
  }
  ExceptionList = local_c;
  return uVar1;
}

