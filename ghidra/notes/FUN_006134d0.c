
void * __thiscall FUN_006134d0(void *this,int param_1)

{
  undefined4 uVar1;
  int iVar2;
  undefined4 *puVar3;
  undefined4 *puVar4;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00657003;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f5c10((undefined4 *)this);
  local_4 = 0;
  FUN_00613ea0((void *)((int)this + 0x7c),0,0);
  *(undefined ***)this = &PTR_FUN_0066e3e0;
  local_4 = CONCAT31(local_4._1_3_,1);
  *(undefined4 *)((int)this + 0x20) = 0;
  *(undefined4 *)((int)this + 0x24) = 0;
  *(undefined4 *)((int)this + 0x28) = 0;
  *(undefined4 *)((int)this + 0x2c) = 0;
  *(undefined4 *)((int)this + 0x30) = 0;
  *(undefined4 *)((int)this + 0x18) = 0;
  *(undefined4 *)((int)this + 0x58) = 0;
  *(undefined4 *)((int)this + 0x68) = 0;
  *(undefined4 *)((int)this + 0x6c) = 0;
  *(undefined4 *)((int)this + 0x38) = 0;
  *(undefined4 *)((int)this + 0x3c) = 0;
  *(undefined4 *)((int)this + 0x4c) = 0;
  *(undefined4 *)((int)this + 0x70) = 0;
  *(undefined4 *)((int)this + 0x74) = 0;
  *(undefined4 *)((int)this + 100) = *(undefined4 *)(param_1 + 100);
  *(undefined4 *)((int)this + 0x5c) = *(undefined4 *)(param_1 + 0x5c);
  *(undefined4 *)((int)this + 0x54) = *(undefined4 *)(param_1 + 0x54);
  *(undefined4 *)((int)this + 0x50) = *(undefined4 *)(param_1 + 0x50);
  puVar3 = (undefined4 *)(param_1 + 0x20);
  puVar4 = (undefined4 *)((int)this + 0x20);
  for (iVar2 = 5; iVar2 != 0; iVar2 = iVar2 + -1) {
    *puVar4 = *puVar3;
    puVar3 = puVar3 + 1;
    puVar4 = puVar4 + 1;
  }
  uVar1 = *(undefined4 *)(param_1 + 0x1c);
  *(undefined4 *)((int)this + 0x78) = 0;
  *(undefined4 *)((int)this + 0x1c) = uVar1;
  *(undefined4 *)((int)this + 0xbc) = *(undefined4 *)(param_1 + 0xbc);
  *(undefined4 *)((int)this + 0xc0) = *(undefined4 *)(param_1 + 0xc0);
  *(undefined4 *)((int)this + 0x40) = 0;
  *(undefined4 *)((int)this + 0x44) = 0;
  *(undefined4 *)((int)this + 0x48) = 0;
  FUN_00613f90((int *)((int)this + 0x34),(undefined4 *)(param_1 + 0x34));
  if (*(int *)((int)this + 0x34) == 0) {
    *(undefined4 *)((int)this + 0x50) = 0;
    *(undefined4 *)((int)this + 0x4c) = 1;
  }
  else if (*(int *)((int)this + 0x54) == 1) {
    iVar2 = FUN_00613cd0((int)this);
    if (iVar2 == 0) {
      iVar2 = FUN_00613d10(this);
      if (iVar2 != 0) {
        *(undefined4 *)((int)this + 0x4c) = 1;
      }
    }
    else {
      *(undefined4 *)((int)this + 0x4c) = 1;
    }
  }
  ExceptionList = local_c;
  return this;
}

