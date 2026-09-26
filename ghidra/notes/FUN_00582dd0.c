
bool __cdecl FUN_00582dd0(uint *param_1,undefined4 *param_2)

{
  bool bVar1;
  undefined4 *puVar2;
  void *this;
  uint *puVar3;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puVar2 = param_2;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064f008;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *param_2 = 0;
  this = (void *)FUN_0051cab0(param_1);
  bVar1 = false;
  if (this != (void *)0x0) {
    puVar3 = (uint *)FUN_00402e40(this,(uint *)&param_2);
    local_4 = 0;
    if ((*puVar3 >> 0x18 < 0x30) || (0x3f < *puVar3 >> 0x18)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    local_4 = 0xffffffff;
    FUN_00619730();
    if (bVar1) {
      *puVar2 = this;
    }
  }
  ExceptionList = local_c;
  return bVar1;
}

