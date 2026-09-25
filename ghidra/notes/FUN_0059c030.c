
void __thiscall FUN_0059c030(void *this,undefined4 *param_1,uint *param_2)

{
  int iVar1;
  void *pvVar2;
  int iVar3;
  uint uVar4;
  undefined4 *puVar5;
  short local_14;
  undefined2 local_12;
  void *pvStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_00651966;
  pvStack_c = ExceptionList;
  iVar3 = 0;
  local_14 = 0;
  local_12 = DAT_0065d428;
  uVar4 = 0x3f2;
  ExceptionList = &pvStack_c;
  iVar1 = FUN_00595c40();
  if (iVar1 == 0) {
    uVar4 = 0x3f0;
  }
  pvStack_10 = (void *)FUN_00618b70(0x114);
  uStack_4 = 0;
  if (pvStack_10 == (void *)0x0) {
    pvVar2 = (void *)0x0;
  }
  else {
    pvVar2 = FUN_00602150(pvStack_10,*param_1,0x128,1,0x31,0x13,this,0xad0a,uVar4,uVar4,param_2,0,0)
    ;
  }
  uStack_4 = 0xffffffff;
  if (pvVar2 != (void *)0x0) {
    local_14 = -0x5265;
    FUN_005ef240(this,pvVar2,0,(int)&local_14);
  }
  uVar4 = 0x15b;
  puVar5 = (undefined4 *)((int)this + 0xa8);
  do {
    pvStack_10 = (void *)FUN_00618b70(0x114);
    uStack_4 = 1;
    if (pvStack_10 == (void *)0x0) {
      pvVar2 = (void *)0x0;
    }
    else {
      pvVar2 = FUN_00602150(pvStack_10,*param_1,uVar4,2,0x19,0xf,this,iVar3 + 0xad0b,iVar3 + 0x3f4,
                            iVar3 + 0x3f8,param_2,0,1);
    }
    uStack_4 = 0xffffffff;
    *puVar5 = pvVar2;
    if (pvVar2 != (void *)0x0) {
      FUN_006030c0(pvVar2,8);
      FUN_00603150((void *)*puVar5,2,0x3fc);
      FUN_00603150((void *)*puVar5,4,iVar3 + 0x3f8);
      local_14 = (short)iVar3 + -0x5263;
      FUN_005ef240(this,(void *)*puVar5,0,(int)&local_14);
      (**(code **)(*(int *)*puVar5 + 0x10))();
    }
    uVar4 = uVar4 + 0x1d;
    iVar3 = iVar3 + 1;
    puVar5 = puVar5 + 1;
  } while (uVar4 < 0x1cf);
  ExceptionList = pvStack_c;
  return;
}

