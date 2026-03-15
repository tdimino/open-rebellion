
void __thiscall FUN_00612480(void *this,int *param_1,int param_2,undefined4 param_3,int param_4)

{
  int iVar1;
  int iVar2;
  undefined4 uVar3;
  
                    /* WARNING: Load size is inaccurate */
  (**(code **)(*this + 4))();
  iVar1 = param_4;
  *(uint *)((int)this + 0x18) = (*(uint *)(param_4 + 4) & 0xffff) + *(int *)(param_4 + 8) * 0x10000;
  if (*(int *)(*(int *)(*(int *)(*param_1 + 4) + 4 + (int)param_1) + 0x4c) != -1) {
    if (param_2 != 0) {
      FUN_00616c30(param_1,param_2);
    }
    iVar2 = FUN_00616d70(param_1);
    *(int *)((int)this + 0x50) = iVar2;
    *(int **)((int)this + 0x4c) = param_1;
    FUN_00612f80((void *)((int)this + 0x20),param_1);
    FUN_00612ce0((void *)((int)this + 0x20),*(int **)((int)this + 0x4c),*(int *)((int)this + 0x50));
    uVar3 = FUN_00612580(this,*(int **)((int)this + 0x4c),&param_3);
    *(undefined4 *)(iVar1 + 0xc) = uVar3;
  }
  return;
}

