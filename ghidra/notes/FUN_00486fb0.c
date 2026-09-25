
void __thiscall FUN_00486fb0(void *this,uint param_1,int param_2,uint *param_3,int param_4)

{
  bool bVar1;
  void *pvVar2;
  undefined4 *puVar3;
  uint *puVar4;
  uint uVar5;
  int *piVar6;
  int iVar7;
  uint local_24;
  undefined4 local_20;
  undefined1 local_1c [4];
  uint local_18 [3];
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  puVar4 = param_3;
  uVar5 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00634a40;
  local_c = ExceptionList;
  if ((int)param_1 < 0x101) {
    if (param_1 == 0x100) {
LAB_0048705a:
      ExceptionList = &local_c;
      FUN_004ece30(&param_3);
      local_4 = 0;
      FUN_0042dbe0(&param_4);
      local_4._0_1_ = 1;
      pvVar2 = (void *)thunk_FUN_005f5060(param_2);
      if (pvVar2 != (void *)0x0) {
        puVar3 = FUN_00403040(pvVar2,&param_1);
        local_4._0_1_ = 2;
        FUN_004f26d0(&param_3,puVar3);
        local_4._0_1_ = 1;
        FUN_00619730();
        if (uVar5 == 0x103) {
          FUN_0041d7f0(&param_3);
        }
        else {
          FUN_0041d6b0(&param_3,&param_4);
        }
      }
      local_4 = (uint)local_4._1_3_ << 8;
      FUN_00619730();
      local_4 = 0xffffffff;
      FUN_00619730();
      ExceptionList = local_c;
      return;
    }
    switch(param_1) {
    case 0x20:
    case 0x25:
      ExceptionList = &local_c;
      FUN_0041d2f0(0);
      ExceptionList = local_c;
      return;
    case 0x21:
    case 0x26:
      ExceptionList = &local_c;
      FUN_00487eb0(this,1);
      ExceptionList = local_c;
      return;
    case 0x22:
    case 0x27:
      ExceptionList = &local_c;
      FUN_00487eb0(this,2);
      ExceptionList = local_c;
      return;
    case 0x23:
    case 0x28:
      ExceptionList = &local_c;
      FUN_00487eb0(this,3);
      ExceptionList = local_c;
      return;
    case 0x24:
    case 0x29:
      ExceptionList = &local_c;
      FUN_00487eb0(this,4);
      ExceptionList = local_c;
      return;
    }
switchD_00486fe8_default:
    ExceptionList = &local_c;
    piVar6 = FUN_00487c50(this,param_1,param_2);
    if (piVar6 == (int *)0x0) {
      ExceptionList = local_c;
      return;
    }
    iVar7 = FUN_00487cc0(this,piVar6);
    if (iVar7 != 0) {
      FUN_0048a340((void *)((int)this + 0x6c),piVar6);
      ExceptionList = local_c;
      return;
    }
    iVar7 = 1;
    goto LAB_0048760f;
  }
  if ((int)param_1 < 0x113) {
    if (0x10f < (int)param_1) {
      ExceptionList = &local_c;
      FUN_00439d10(*(void **)((int)this + 0xc0),param_1);
      FUN_0041d5e0(0);
      ExceptionList = local_c;
      return;
    }
    if (param_1 == 0x103) goto LAB_0048705a;
    goto switchD_00486fe8_default;
  }
  if ((int)param_1 < 0x171) {
    if (param_1 == 0x170) {
      return;
    }
    switch(param_1) {
    case 0x113:
      ExceptionList = &local_c;
      pvVar2 = (void *)FUN_004f3d70(*(int *)((int)this + 0x24));
      if (pvVar2 == (void *)0x0) {
        ExceptionList = local_c;
        return;
      }
      puVar4 = FUN_004025b0(pvVar2,&local_24);
      local_4 = 6;
      FUN_0041d7f0(puVar4);
      local_4 = 0xffffffff;
      FUN_00619730();
      ExceptionList = local_c;
      return;
    case 0x114:
      ExceptionList = &local_c;
      puVar3 = (undefined4 *)FUN_004ece30(&local_20);
      local_4 = 7;
      FUN_0041d7f0(puVar3);
      local_4 = 0xffffffff;
      FUN_00619730();
      ExceptionList = local_c;
      return;
    case 0x115:
    case 0x116:
    case 0x117:
    case 0x118:
      ExceptionList = &local_c;
      FUN_00439d60(*(void **)((int)this + 0xc0),param_1);
      ExceptionList = local_c;
      return;
    case 0x119:
    case 0x11a:
    case 0x11b:
    case 0x11e:
      ExceptionList = &local_c;
      FUN_00439e80(param_1);
      ExceptionList = local_c;
      return;
    case 0x11c:
      ExceptionList = &local_c;
      FUN_0041d770(1,0x79);
      ExceptionList = local_c;
      return;
    }
    goto switchD_00486fe8_default;
  }
  if ((int)param_1 < 0x203) {
    if ((int)param_1 < 0x201) {
      switch(param_1) {
      case 0x171:
        ExceptionList = &local_c;
        uVar5 = FUN_0048a1c0((void *)((int)this + 0x6c),1);
        FUN_0048a210((void *)((int)this + 0x6c),1,uVar5 ^ 4);
        ExceptionList = local_c;
        return;
      case 0x172:
        ExceptionList = &local_c;
        uVar5 = FUN_0048a1c0((void *)((int)this + 0x6c),0x80);
        FUN_0048a210((void *)((int)this + 0x6c),0x80,uVar5 ^ 4);
        ExceptionList = local_c;
        return;
      case 0x173:
        ExceptionList = &local_c;
        uVar5 = FUN_0048a1c0((void *)((int)this + 0x6c),0x10);
        FUN_0048a210((void *)((int)this + 0x6c),0x10,uVar5 ^ 4);
        ExceptionList = local_c;
        return;
      case 0x174:
        ExceptionList = &local_c;
        uVar5 = FUN_0048a1c0((void *)((int)this + 0x6c),4);
        FUN_0048a210((void *)((int)this + 0x6c),4,uVar5 ^ 4);
        ExceptionList = local_c;
        return;
      case 0x175:
        ExceptionList = &local_c;
        uVar5 = FUN_0048a1c0((void *)((int)this + 0x6c),8);
        FUN_0048a210((void *)((int)this + 0x6c),8,uVar5 ^ 4);
        ExceptionList = local_c;
        return;
      case 0x176:
        ExceptionList = &local_c;
        uVar5 = FUN_0048a1c0((void *)((int)this + 0x6c),0x40);
        FUN_0048a210((void *)((int)this + 0x6c),0x40,uVar5 ^ 4);
        ExceptionList = local_c;
        return;
      case 0x177:
        ExceptionList = &local_c;
        uVar5 = FUN_0048a1c0((void *)((int)this + 0x6c),0x100);
        FUN_0048a210((void *)((int)this + 0x6c),0x100,uVar5 ^ 4);
        ExceptionList = local_c;
        return;
      case 0x178:
        ExceptionList = &local_c;
        uVar5 = FUN_0048a1c0((void *)((int)this + 0x6c),0x200);
        FUN_0048a210((void *)((int)this + 0x6c),0x200,uVar5 ^ 4);
        ExceptionList = local_c;
        return;
      case 0x179:
        ExceptionList = &local_c;
        uVar5 = FUN_0048a1c0((void *)((int)this + 0x6c),0x20);
        FUN_0048a210((void *)((int)this + 0x6c),0x20,uVar5 ^ 4);
        ExceptionList = local_c;
        return;
      }
      goto switchD_00486fe8_default;
    }
    ExceptionList = &local_c;
    piVar6 = FUN_00487c50(this,param_1,param_2);
    if (piVar6 == (int *)0x0) {
      ExceptionList = local_c;
      return;
    }
    if ((*puVar4 & 0xff000000) == 0) {
      FUN_0041d5e0(piVar6);
      ExceptionList = local_c;
      return;
    }
    iVar7 = FUN_00487cc0(this,piVar6);
    if (iVar7 != 0) {
      FUN_0048a340((void *)((int)this + 0x6c),piVar6);
      ExceptionList = local_c;
      return;
    }
    goto LAB_0048746a;
  }
  switch(param_1) {
  case 0x203:
    goto LAB_00487439;
  default:
    goto switchD_00486fe8_default;
  case 0x210:
    ExceptionList = &local_c;
    piVar6 = FUN_00487c50(this,param_1,param_2);
    if (piVar6 != (int *)0x0) {
      FUN_0041d640((int)piVar6);
    }
    break;
  case 0x211:
    ExceptionList = &local_c;
    piVar6 = FUN_00487c50(this,param_1,param_2);
    if (piVar6 != (int *)0x0) {
      FUN_0041d640((int)piVar6);
    }
    break;
  case 0x212:
    ExceptionList = &local_c;
    piVar6 = FUN_00487c50(this,param_1,param_2);
    if (piVar6 != (int *)0x0) {
      FUN_0041d640((int)piVar6);
    }
    break;
  case 0x214:
    ExceptionList = &local_c;
    piVar6 = FUN_00487c50(this,param_1,param_2);
    if (piVar6 == (int *)0x0) {
      ExceptionList = local_c;
      return;
    }
    if ((*puVar4 & 0xff000000) == 0) {
      FUN_0041d5e0(piVar6);
      ExceptionList = local_c;
      return;
    }
    iVar7 = 0;
    goto LAB_0048760f;
  case 0x215:
LAB_00487439:
    ExceptionList = &local_c;
    piVar6 = FUN_00487c50(this,param_1,param_2);
    if (piVar6 == (int *)0x0) {
      ExceptionList = local_c;
      return;
    }
    if (param_4 == 0) {
      FUN_0041d600(piVar6);
      ExceptionList = local_c;
      return;
    }
    FUN_005f3090(piVar6 + 0x11,param_4);
LAB_0048746a:
    iVar7 = 0;
LAB_0048760f:
    FUN_00487740(this,piVar6,iVar7);
    break;
  case 0x240:
    ExceptionList = &local_c;
    piVar6 = FUN_00487c50(this,param_1,param_2);
    if (piVar6 != (int *)0x0) {
      FUN_0041d5e0(piVar6);
    }
    break;
  case 0x270:
    ExceptionList = &local_c;
    FUN_004ece30(&param_3);
    local_4 = 3;
    pvVar2 = (void *)thunk_FUN_005f5060(param_2);
    if (pvVar2 != (void *)0x0) {
      puVar3 = FUN_00403040(pvVar2,local_1c);
      local_4._0_1_ = 4;
      FUN_004f26d0(&param_3,puVar3);
      local_4 = CONCAT31(local_4._1_3_,3);
      FUN_00619730();
      piVar6 = FUN_004f2d10(*(int *)((int)this + 0x24),(uint *)&param_3);
      if (piVar6 != (int *)0x0) {
        do {
          local_18[1] = 0x90;
          local_18[2] = 0x98;
          if (((uint)param_3 >> 0x18 < 0x90) || (0x97 < (uint)param_3 >> 0x18)) {
            bVar1 = false;
          }
          else {
            bVar1 = true;
          }
          FUN_00619730();
          if (bVar1) break;
          piVar6 = (int *)piVar6[7];
          if (piVar6 == (int *)0x0) goto LAB_004875a9;
          puVar4 = FUN_004025b0(piVar6,local_18);
          local_4._0_1_ = 5;
          FUN_004f26d0(&param_3,puVar4);
          local_4 = CONCAT31(local_4._1_3_,3);
          FUN_00619730();
        } while (piVar6 != (int *)0x0);
        if ((piVar6 != (int *)0x0) &&
           (piVar6 = FUN_00487c50(this,uVar5,param_2), piVar6 != (int *)0x0)) {
          FUN_00487740(this,piVar6,0);
        }
      }
    }
LAB_004875a9:
    local_4 = 0xffffffff;
    FUN_00619730();
  }
  ExceptionList = local_c;
  return;
}

