
void * __thiscall FUN_00616ae0(void *this,int *param_1)

{
  int *piVar1;
  int iVar2;
  uint uVar3;
  
  iVar2 = FUN_00615fc0((int *)this);
  if (iVar2 != 0) {
    uVar3 = FUN_00618440(param_1);
    if (uVar3 != 0xffffffff) {
      while( true ) {
                    /* WARNING: Load size is inaccurate */
        piVar1 = *(int **)(*(int *)(*this + 4) + 4 + (int)this);
        if ((undefined1 *)piVar1[7] < (undefined1 *)piVar1[8]) {
          *(undefined1 *)piVar1[7] = (char)uVar3;
          uVar3 = uVar3 & 0xff;
          piVar1[7] = piVar1[7] + 1;
        }
        else {
          uVar3 = (**(code **)(*piVar1 + 0x1c))(uVar3);
        }
        if (uVar3 == 0xffffffff) break;
        uVar3 = FUN_00618440(param_1);
        if (uVar3 == 0xffffffff) {
          FUN_00616040((int *)this);
          return this;
        }
      }
                    /* WARNING: Load size is inaccurate */
      *(uint *)(*(int *)(*this + 4) + 8 + (int)this) =
           *(uint *)(*(int *)(*this + 4) + 8 + (int)this) | 2;
    }
    FUN_00616040((int *)this);
  }
  return this;
}

