
void __thiscall
FUN_004ab1f0(void *this,undefined4 param_1,undefined4 param_2,undefined4 param_3,int param_4,
            int param_5,int param_6,int param_7,uint param_8,uint param_9,uint param_10,
            uint param_11)

{
  void *pvVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063838b;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  pvVar1 = (void *)FUN_00618b70(0x78);
  local_4 = 0;
  if (pvVar1 == (void *)0x0) {
    pvVar1 = (void *)0x0;
  }
  else {
    pvVar1 = FUN_004ab560(pvVar1,param_1,param_2,param_4,param_5,param_6,param_7,param_8,param_9,
                          param_10,param_11,param_3,this,(void *)0x1,(void *)0x0);
  }
  local_4 = 0xffffffff;
  if (pvVar1 != (void *)0x0) {
    FUN_005fe040(pvVar1,*(undefined4 *)((int)this + 0xa4));
    FUN_005f58d0((void *)((int)this + 0x138),(int)pvVar1);
  }
  ExceptionList = local_c;
  return;
}

