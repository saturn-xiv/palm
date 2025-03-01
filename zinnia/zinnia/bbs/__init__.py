from flask import (Blueprint, render_template)

router = Blueprint('bbs', __name__, url_prefix='/bbs')


@router.route('/', methods=['GET'])
def index():
    return render_template('bbs/index.html')
