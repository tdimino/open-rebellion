
void * __thiscall
FUN_00613360(void *this,undefined4 param_1,undefined4 param_2,undefined4 param_3,undefined4 param_4,
            undefined4 param_5)

{
  undefined4 *puVar1;
  int *piVar2;
  int iVar3;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00656fe3;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f5c10(this);
  local_4 = 0;
  FUN_00613ea0((void *)((int)this + 0x7c),0,0);
  puVar1 = (undefined4 *)((int)this + 0x20);
  *(undefined ***)this = &PTR_FUN_0066e3e0;
  piVar2 = (int *)((int)this + 0x34);
  *puVar1 = 0;
  local_4 = CONCAT31(local_4._1_3_,1);
  *(undefined4 *)((int)this + 0x24) = 0;
  *(undefined4 *)((int)this + 0x28) = 0;
  *(undefined4 *)((int)this + 0x2c) = 0;
  *(undefined4 *)((int)this + 0x30) = 0;
  *(undefined4 *)((int)this + 0x1c) = param_1;
  *(undefined4 *)((int)this + 0x54) = param_4;
  *(undefined4 *)((int)this + 100) = param_5;
  *(undefined4 *)((int)this + 0x60) = param_5;
  *(undefined4 *)((int)this + 0x18) = 0;
  *(undefined4 *)((int)this + 0x5c) = 0;
  *(undefined4 *)((int)this + 0x58) = 0;
  *(undefined4 *)((int)this + 0x68) = 0;
  *piVar2 = 0;
  *(undefined4 *)((int)this + 0x38) = 0;
  *(undefined4 *)((int)this + 0x3c) = 0;
  *(undefined4 *)((int)this + 0x4c) = 0;
  *(undefined4 *)((int)this + 0x6c) = 0;
  *(undefined4 *)((int)this + 0x70) = 0;
  *(undefined4 *)((int)this + 0x74) = 0;
  *(undefined4 *)((int)this + 0x78) = 0;
  *puVar1 = 0x14;
  *(undefined4 *)((int)this + 0x30) = param_2;
  *(undefined4 *)((int)this + 0x28) = param_3;
  *(undefined4 *)((int)this + 0x24) = 0x181e8;
  *(undefined4 *)((int)this + 0x40) = 0;
  *(undefined4 *)((int)this + 0x44) = 0;
  *(undefined4 *)((int)this + 0x48) = 0;
  if (*(int *)((int)this + 0x54) == 2) {
    *(undefined4 *)((int)this + 0x5c) = 1;
    *(uint *)((int)this + 0x24) = *(uint *)((int)this + 0x24) | 2;
    *(undefined4 *)((int)this + 100) = *(undefined4 *)((int)this + 0x50);
  }
  *(undefined4 *)((int)this + 0xbc) = 0;
  *(undefined4 *)((int)this + 0xc0) = 0;
  *piVar2 = 0;
  FUN_00613f70(puVar1,piVar2);
  if (*piVar2 == 0) {
    *(undefined4 *)((int)this + 0x50) = 0;
    *(undefined4 *)((int)this + 0x4c) = 1;
  }
  else {
    *(undefined4 *)((int)this + 0x50) = param_3;
    if (*(int *)((int)this + 0x54) == 1) {
      iVar3 = FUN_00613cd0((int)this);
      if (iVar3 == 0) {
        iVar3 = FUN_00613d10(this);
        if (iVar3 != 0) {
          *(undefined4 *)((int)this + 0x4c) = 1;
        }
      }
      else {
        *(undefined4 *)((int)this + 0x4c) = 1;
      }
    }
  }
  ExceptionList = local_c;
  return this;
}

