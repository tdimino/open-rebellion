
void __thiscall FUN_00612510(void *this,uint param_1,undefined4 param_2,int param_3)

{
  uint uVar1;
  int iVar2;
  undefined4 uVar3;
  
                    /* WARNING: Load size is inaccurate */
  (**(code **)(*this + 4))();
  iVar2 = param_3;
  uVar1 = param_1;
  *(uint *)((int)this + 0x18) = (*(uint *)(param_3 + 4) & 0xffff) + *(int *)(param_3 + 8) * 0x10000;
  if (param_1 != 0) {
    FUN_00613150((void *)((int)this + 0x20),param_1);
    param_1 = uVar1;
    FUN_00612e20((void *)((int)this + 0x20),&param_1);
    uVar3 = FUN_00612600(this,param_1,&param_2);
    *(undefined4 *)(iVar2 + 0xc) = uVar3;
  }
  return;
}

