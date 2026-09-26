
void * __thiscall FUN_00615f00(void *this,uint param_1)

{
  int *piVar1;
  int iVar2;
  uint uVar3;
  uint uVar4;
  
  iVar2 = FUN_00615fc0(this);
  if (iVar2 != 0) {
    if (*(int *)(*(int *)(*(int *)this + 4) + 0x30 + (int)this) != 0) {
      param_1._0_2_ = (ushort)(byte)param_1;
      FUN_006162b0(this,&DAT_006b120c,(char *)&param_1);
      FUN_00616040(this);
      return this;
    }
    piVar1 = *(int **)((int)this + *(int *)(*(int *)this + 4) + 4);
    uVar4 = param_1 & 0xff;
    if ((byte *)piVar1[7] < (byte *)piVar1[8]) {
      *(byte *)piVar1[7] = (byte)param_1;
      piVar1[7] = piVar1[7] + 1;
      uVar3 = uVar4;
    }
    else {
      uVar3 = (**(code **)(*piVar1 + 0x1c))(uVar4);
    }
    if (uVar3 == 0xffffffff) {
      iVar2 = (**(code **)(**(int **)(*(int *)(*(int *)this + 4) + 4 + (int)this) + 0x1c))(uVar4);
      if (iVar2 == -1) {
        *(uint *)(*(int *)(*(int *)this + 4) + 8 + (int)this) =
             *(uint *)(*(int *)(*(int *)this + 4) + 8 + (int)this) | 6;
      }
    }
    FUN_00616040(this);
  }
  return this;
}

