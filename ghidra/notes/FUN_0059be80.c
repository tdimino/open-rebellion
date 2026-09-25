
void __thiscall FUN_0059be80(void *this,undefined4 *param_1,uint *param_2)

{
  int iVar1;
  void *pvVar2;
  uint uVar3;
  undefined4 *puVar4;
  int iVar5;
  undefined2 local_18;
  undefined2 local_16;
  short local_14;
  undefined2 local_12;
  void *pvStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_00651946;
  pvStack_c = ExceptionList;
  iVar5 = 0;
  local_14 = 0;
  local_12 = DAT_0065d428;
  local_18 = 0;
  local_16 = DAT_0065d428;
  uVar3 = 0x3ec;
  ExceptionList = &pvStack_c;
  iVar1 = FUN_00595c40();
  if (iVar1 == 0) {
    uVar3 = 0x3e9;
  }
  pvStack_10 = (void *)FUN_00618b70(0x114);
  uStack_4 = 0;
  if (pvStack_10 == (void *)0x0) {
    pvVar2 = (void *)0x0;
  }
  else {
    pvVar2 = FUN_00602150(pvStack_10,*param_1,9,1,0x31,0x13,this,0xad01,uVar3,uVar3,param_2,0,0);
  }
  uStack_4 = 0xffffffff;
  if (pvVar2 != (void *)0x0) {
    local_18 = 0xad91;
    FUN_005ef240(this,pvVar2,0,(int)&local_18);
  }
  uVar3 = 0x3c;
  puVar4 = (undefined4 *)((int)this + 0xb8);
  do {
    pvStack_10 = (void *)FUN_00618b70(0x120);
    uStack_4 = 1;
    if (pvStack_10 == (void *)0x0) {
      pvVar2 = (void *)0x0;
    }
    else {
      pvVar2 = FUN_005c52e0(pvStack_10,*param_1,uVar3,2,0x19,0x11,this,iVar5 + 0xad02,0x3ed,0x3ee,
                            0x3ef,0x3ef,param_2,0,1);
    }
    uStack_4 = 0xffffffff;
    *puVar4 = pvVar2;
    if (pvVar2 != (void *)0x0) {
      FUN_006030c0(pvVar2,8);
      FUN_00603150((void *)*puVar4,2,0x3ef);
      FUN_00603150((void *)*puVar4,4,0x3ee);
      FUN_00603010((void *)*puVar4,4,0x2ffffff);
      FUN_005c54d0((void *)*puVar4);
      local_14 = (short)iVar5 + -0x527f;
      local_18 = 0xad92;
    }
    FUN_005ef240(this,(void *)*puVar4,(int)&local_14,(int)&local_18);
    uVar3 = uVar3 + 0x1d;
    iVar5 = iVar5 + 1;
    puVar4 = puVar4 + 1;
  } while (uVar3 < 0x124);
  ExceptionList = pvStack_c;
  return;
}

