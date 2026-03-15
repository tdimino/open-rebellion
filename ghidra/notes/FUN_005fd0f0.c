
undefined4 __thiscall FUN_005fd0f0(void *this,uint *param_1,int param_2,int param_3)

{
  bool bVar1;
  undefined3 extraout_var;
  uint uVar2;
  undefined4 uVar3;
  
  uVar3 = 0;
  bVar1 = FUN_005fc040((int *)this);
  if (CONCAT31(extraout_var,bVar1) != 0) {
    uVar2 = FUN_005fc100((uint *)this);
    if (uVar2 == 8) {
      uVar3 = FUN_005fc710(this,param_1,param_2,param_3,(uint)*(byte *)((int)this + 8),0,(byte *)0x0
                           ,0,0);
      return uVar3;
    }
    FUN_0060e710(this,param_1,param_2,param_3,
                 CONCAT13((char)((uint)((int)this + 9) >> 0x18),*(undefined3 *)((int)this + 9)),0,
                 (byte *)0x0,0,0);
    uVar3 = 1;
  }
  return uVar3;
}

