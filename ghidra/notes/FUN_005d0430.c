
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __thiscall FUN_005d0430(void *this,float param_1)

{
  int iVar1;
  float fVar2;
  float fVar3;
  
  if (*(int *)(*(int *)((int)this + 8) + 0x34) == 0) {
    if (*(int *)((int)this + 0x10) == 0) {
      *(undefined4 *)((int)this + 0x10) = 1;
      (**(code **)(*(int *)this + 8))();
    }
    if ((*(int *)((int)this + 0x34) == 0) && (iVar1 = *(int *)((int)this + 0x2c), iVar1 != 2)) {
      if (iVar1 == 9) {
        FUN_005d03f0((int)this);
        fVar3 = _DAT_0066d090;
        fVar2 = *(float *)((int)this + 0x30) - param_1;
        *(float *)((int)this + 0x30) = fVar2;
        if (fVar2 <= fVar3) {
          FUN_005cfe80(this,6);
          *(undefined4 *)((int)this + 0x30) = DAT_0066d084;
        }
      }
      else if (iVar1 == 10) {
        FUN_005d03f0((int)this);
        fVar3 = _DAT_0066d090;
        fVar2 = *(float *)((int)this + 0x30) - param_1;
        *(float *)((int)this + 0x30) = fVar2;
        if (fVar2 <= fVar3) {
          FUN_005cfe80(this,7);
          *(undefined4 *)((int)this + 0x30) = DAT_0066d084;
          return;
        }
      }
    }
  }
  return;
}

