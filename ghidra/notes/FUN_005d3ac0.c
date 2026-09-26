
void __thiscall FUN_005d3ac0(void *this,undefined1 *param_1,int param_2)

{
  int *piVar1;
  undefined1 *puVar2;
  void *pvVar3;
  int iVar4;
  undefined4 uVar5;
  int iVar6;
  undefined2 extraout_var;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  iVar4 = param_2;
  puVar2 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006546bb;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  if (((param_2 != 0) && (ExceptionList = &pvStack_c, param_1 == &DAT_00000002)) &&
     (ExceptionList = &pvStack_c, (*(byte *)((int)this + 0x120) & 1) != 0)) {
    ExceptionList = &pvStack_c;
    (**(code **)(*(int *)this + 0x24))(1,0);
  }
  if (iVar4 == 0) {
    *(uint *)((int)this + 0x120) = *(uint *)((int)this + 0x120) & ~(uint)puVar2;
    if (*(int *)((int)this + 0x138) != 0) {
      iVar6 = (**(code **)(**(int **)((int)this + 0x124) + 0xa4))
                        (*(int **)((int)this + 0x124),*(int *)((int)this + 0x138),0);
      FUN_005d8b00(iVar6);
    }
    *(undefined4 *)((int)this + 0x138) = 0;
    *(undefined4 *)((int)this + 300) = 0;
    if (((puVar2 == (undefined1 *)0x1) && ((*(byte *)((int)this + 0x120) & 2) == 0)) ||
       ((puVar2 == &DAT_00000002 && ((*(byte *)((int)this + 0x120) & 1) == 0)))) {
      FUN_005f5970((void *)(DAT_006bcbd4 + 0x604),*(int *)((int)this + 0x18));
    }
  }
  else {
    if (puVar2 == (undefined1 *)(*(uint *)((int)this + 0x120) & 3)) {
      ExceptionList = pvStack_c;
      return;
    }
    if ((*(uint *)((int)this + 0x120) & 3) == 0) {
      param_1 = (undefined1 *)FUN_00618b70(0x14);
      local_4 = 0;
      if (param_1 == (undefined1 *)0x0) {
        pvVar3 = (void *)0x0;
      }
      else {
        pvVar3 = FUN_005ee4f0(param_1,(int)this);
      }
      local_4 = 0xffffffff;
      FUN_005f58d0((void *)(DAT_006bcbd4 + 0x604),(int)pvVar3);
    }
    if (puVar2 == &DAT_00000002) {
      *(undefined4 *)((int)this + 0x134) = 0xdc0;
    }
    if (puVar2 == (undefined1 *)0x1) {
      *(undefined4 *)((int)this + 0x134) = 0xe24;
    }
    *(undefined4 *)((int)this + 0x130) = 0;
    *(uint *)((int)this + 0x120) = *(uint *)((int)this + 0x120) | (uint)puVar2;
    FUN_005fbbb0((int)this + 0x108);
    param_1 = (undefined1 *)
              CONCAT22(extraout_var,*(short *)((int)this + 0x134) + *(short *)((int)this + 0x130));
    piVar1 = (int *)(DAT_006bcbd4 + 0x168);
    for (iVar4 = FUN_005f5060(DAT_006bcbd4 + 0x16c); iVar4 != 0; iVar4 = *(int *)(iVar4 + 0x10)) {
      if (*(short *)(iVar4 + 0x18) == (short)param_1) goto LAB_005d3bd3;
    }
    iVar4 = 0;
LAB_005d3bd3:
    if (iVar4 == 0) {
      uVar5 = (**(code **)(*piVar1 + 4))(&param_1);
      *(undefined4 *)((int)this + 0x138) = uVar5;
    }
    else {
      *(undefined4 *)((int)this + 0x138) = *(undefined4 *)(iVar4 + 0x1c);
    }
    iVar4 = (**(code **)(**(int **)((int)this + 0x124) + 0x48))
                      (*(int **)((int)this + 0x124),*(undefined4 *)((int)this + 0x138),0);
    FUN_005d8b00(iVar4);
    iVar4 = param_2;
  }
  if (((iVar4 == 0) && (puVar2 == &DAT_00000002)) && ((*(byte *)((int)this + 0x120) & 1) != 0)) {
    (**(code **)(*(int *)this + 0x24))(1,1);
  }
  ExceptionList = pvStack_c;
  return;
}

