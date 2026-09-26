
void __thiscall FUN_00476ba0(void *this,int *param_1)

{
  int iVar1;
  void *pvVar2;
  uint uVar3;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00632f8b;
  local_c = ExceptionList;
  if (param_1 != (int *)0x0) {
    uVar3 = param_1[6];
    ExceptionList = &local_c;
    *(undefined4 *)((int)this + 0xbc) = 0;
    *(undefined4 *)((int)this + 0xc0) = 0;
    iVar1 = FUN_005f5500((void *)((int)this + 0xb0),uVar3);
    if (iVar1 == 0) {
      pvVar2 = (void *)FUN_00618b70(0x1c);
      local_4 = 0;
      if (pvVar2 == (void *)0x0) {
        pvVar2 = (void *)0x0;
      }
      else {
        pvVar2 = FUN_005f55d0(pvVar2,uVar3);
      }
      local_4 = 0xffffffff;
      if (pvVar2 != (void *)0x0) {
        FUN_005f5440((void *)((int)this + 0xb0),pvVar2);
        uVar3 = FUN_004794b0((int)param_1);
        *(int *)((int)this + (uVar3 & 0xffff) * 4 + 0xa0) =
             *(int *)((int)this + (uVar3 & 0xffff) * 4 + 0xa0) + 1;
        param_1[0x1c] = *(int *)((int)this + 0x18);
        FUN_004f26d0(param_1 + 0x1d,(undefined4 *)((int)this + 0x34));
        (**(code **)(*param_1 + 0x1c))(*(undefined4 *)((int)this + 0x44));
      }
    }
  }
  ExceptionList = local_c;
  return;
}

