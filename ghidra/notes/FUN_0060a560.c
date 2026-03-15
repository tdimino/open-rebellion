
int __thiscall
FUN_0060a560(void *this,undefined4 param_1,undefined4 param_2,undefined2 param_3,undefined2 param_4,
            undefined4 param_5,undefined4 param_6,undefined4 param_7,undefined4 param_8,
            uint *param_9)

{
  void *pvVar1;
  int iVar2;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006568ab;
  local_c = ExceptionList;
  iVar2 = 0;
  ExceptionList = &local_c;
  pvVar1 = (void *)FUN_00618b70(0xf0);
  local_4 = 0;
  if (pvVar1 == (void *)0x0) {
    pvVar1 = (void *)0x0;
  }
  else {
    pvVar1 = FUN_006100d0(pvVar1,param_1,1,1,1,1,param_2,this,param_3,param_4,param_5,param_6,
                          param_7,param_8);
  }
  local_4 = 0xffffffff;
  *(void **)((int)this + 8) = pvVar1;
  if (pvVar1 != (void *)0x0) {
    FUN_005ffce0(pvVar1,0);
    iVar2 = FUN_0060b4b0(*(int **)((int)this + 8));
    FUN_0060b2c0(*(void **)((int)this + 8),param_9);
    if ((iVar2 != 0) && (*(int **)((int)this + 4) != (int *)0x0)) {
      iVar2 = FUN_0060b4b0(*(int **)((int)this + 4));
    }
  }
  ExceptionList = local_c;
  return iVar2;
}

