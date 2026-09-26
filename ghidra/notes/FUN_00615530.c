
void * __thiscall FUN_00615530(void *this,char *param_1,int param_2,int param_3)

{
  char cVar1;
  uint uVar2;
  char *pcVar3;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00657098;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_00617d60(this);
  *(undefined ***)this = &PTR_FUN_0066e4e4;
  local_4 = 0;
  *(undefined4 *)((int)this + 0x58) = 1;
  *(undefined4 *)((int)this + 0x4c) = 0;
  if (param_2 == 0) {
    uVar2 = 0xffffffff;
    pcVar3 = param_1;
    do {
      if (uVar2 == 0) break;
      uVar2 = uVar2 - 1;
      cVar1 = *pcVar3;
      pcVar3 = pcVar3 + 1;
    } while (cVar1 != '\0');
    pcVar3 = param_1 + (~uVar2 - 1);
  }
  else if (param_2 < 0) {
    pcVar3 = (char *)0xffffffff;
  }
  else {
    pcVar3 = param_1 + param_2;
  }
  FUN_00618020(this,param_1,pcVar3,0);
  *(char **)((int)this + 0x24) = param_1;
  *(char **)((int)this + 0x28) = param_1;
  if (param_3 == 0) {
    *(char **)((int)this + 0x2c) = pcVar3;
    *(undefined4 *)((int)this + 0xc) = 0xffffffff;
    *(undefined4 *)((int)this + 0x18) = 0;
    *(undefined4 *)((int)this + 0x1c) = 0;
    *(undefined4 *)((int)this + 0x20) = 0;
  }
  else {
    *(int *)((int)this + 0x2c) = param_3;
    *(undefined4 *)((int)this + 0xc) = 0xffffffff;
    *(int *)((int)this + 0x18) = param_3;
    *(int *)((int)this + 0x1c) = param_3;
    *(char **)((int)this + 0x20) = pcVar3;
  }
  ExceptionList = local_c;
  return this;
}

